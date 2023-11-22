#[macro_use]
    extern crate serde;
    use candid::{Decode, Encode};
    use ic_cdk::api::time;
    use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
    use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
    use std::{borrow::Cow, cell::RefCell};
    
    type Memory = VirtualMemory<DefaultMemoryImpl>;
    type IdCell = Cell<u64, Memory>;
    
    #[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
    struct User {
        id: u64,
        name: String,
        weight: u64,
        height: u64,
        age: u64,
        created_at: u64,
        updated_at: Option<u64>,
    }

    // a trait that must be implemented for a struct that is stored in a stable struct
    impl Storable for User {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }
    
        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }
    }
    
    // another trait that must be implemented for a struct that is stored in a stable struct
    impl BoundedStorable for User {
        const MAX_SIZE: u32 = 1024;
        const IS_FIXED_SIZE: bool = false;
    }

    #[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
    struct WorkoutPlan {
        id: u64,
        user_id: u64,
        push_ups: u64,
        sit_ups: u64,
        running_time: u64,
        created_at: u64,
        updated_at: Option<u64>,
    }
    
    // a trait that must be implemented for a struct that is stored in a stable struct
    impl Storable for WorkoutPlan {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }
    
        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }
    }
    
    // another trait that must be implemented for a struct that is stored in a stable struct
    impl BoundedStorable for WorkoutPlan {
        const MAX_SIZE: u32 = 1024;
        const IS_FIXED_SIZE: bool = false;
    }
    
    thread_local! {
        static USER_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
            MemoryManager::init(DefaultMemoryImpl::default())
        );

        static WORK_OUT_PLAN_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
            MemoryManager::init(DefaultMemoryImpl::default())
        );
    
        static USER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
            IdCell::init(USER_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
                .expect("Cannot create a counter")
        );

        static WORK_OUT_PLAN_ID_COUNTER: RefCell<IdCell> = RefCell::new(
            IdCell::init(WORK_OUT_PLAN_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
                .expect("Cannot create a counter")
        );
    
        static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
            RefCell::new(StableBTreeMap::init(
                USER_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        ));

        static WORKOUT_PLAN_STORAGE: RefCell<StableBTreeMap<u64, WorkoutPlan, Memory>> =
            RefCell::new(StableBTreeMap::init(
                WORK_OUT_PLAN_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        ));
    }
    
    #[derive(candid::CandidType, Serialize, Deserialize, Default)]
    struct UserPayload {
        name: String,
        weight: u64,
        height: u64,
        age: u64,
    }
    
    #[derive(candid::CandidType, Serialize, Deserialize, Clone, Default)]
    struct WorkoutPlanPayload {
        user_id: u64,
        push_ups: u64,
        sit_ups: u64,
        running_time: u64,
    }

    #[ic_cdk::query]
    fn get_user(id: u64) -> Result<User, Error> {
        match _get_user(&id) {
            Some(user) => Ok(user),
            None => Err(Error::NotFound {
                msg: format!("a user with id={} not found", id),
            }),
        }
    }
    
    #[ic_cdk::update]
    fn add_user(user: UserPayload) -> Option<User> {
        let id = USER_ID_COUNTER
            .with(|counter| {
                let current_value = *counter.borrow().get();
                counter.borrow_mut().set(current_value + 1)
            })
            .expect("cannot increment id counter");
        let user = User {
            id,
            name: user.name,
            weight: user.weight,
            height: user.height,
            age: user.age,
            created_at: time(),
            updated_at: None,
        };
        do_insert_user(&user);
        Some(user)
    }
    
    #[ic_cdk::update]
    fn update_user(id: u64, payload: UserPayload) -> Result<User, Error> {
        match USER_STORAGE.with(|service| service.borrow().get(&id)) {
            Some(mut user) => {
                user.name = payload.name;
                user.weight = payload.weight;
                user.height = payload.height;
                user.age = payload.age;
                user.updated_at = Some(time());
                do_insert_user(&user);
                Ok(user)
            }
            None => Err(Error::NotFound {
                msg: format!(
                    "couldn't update a user with id={}. user not found",
                    id
                ),
            }),
        }
    }
    
    // helper method to perform insert.
    fn do_insert_user(user: &User) {
        USER_STORAGE.with(|service| service.borrow_mut().insert(user.id, user.clone()));
    }
    
    #[ic_cdk::update]
    fn delete_user(id: u64) -> Result<User, Error> {
        match USER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
            Some(user) => Ok(user),
            None => Err(Error::NotFound {
                msg: format!(
                    "couldn't delete a user with id={}. user not found.",
                    id
                ),
            }),
        }
    }

    #[ic_cdk::update]
    fn generate_workout_plan(user_id: u64) -> Result<WorkoutPlan, Error> {
        let user = _get_user(&user_id);
        if check_user_wp(user_id) {
            return Err(Error::Exists {
                msg: format!(
                    "Work plan for user with id {} exists!",
                    user_id
                ),
            })
        }
        let id = WORK_OUT_PLAN_ID_COUNTER
            .with(|counter| {
                let current_value = *counter.borrow().get();
                counter.borrow_mut().set(current_value + 1)
            })
            .expect("cannot increment workout plan id counter");
        match user {
            Some(_user) => {
                let wp = _gen_wp(&user_id).workout_plan(user_id);
                match wp {
                    Some(my_wp) => {
                        let workp = WorkoutPlan {
                            id: id,
                            user_id: user_id,
                            push_ups: my_wp.push_ups,
                            sit_ups: my_wp.sit_ups,
                            running_time: my_wp.running_time,
                            created_at: time(),
                            updated_at: None,
                        };
                        do_insert_wp(&workp);
                        Ok(workp) 
                    },
                    None => Err(Error::NotFound {
                        msg: format!(
                            "couldn't generate workout for user with id={}. Error generating workout plan",
                            user_id
                        ),
                    }),
                }
                       
            },
            None => Err(Error::NotFound {
                msg: format!(
                    "couldn't find user with id={}. user not found",
                    user_id
                ),
            }),
        }
    }

    #[ic_cdk::query]
    fn get_user_workout_plan(user_id: u64) -> Result<WorkoutPlan, Error> {
        match _get_workout(&user_id) {
            Some((_i, wp)) => Ok(wp),
            None => Err(Error::NotFound {
                msg: format!("Workout plan for user with id={} not found", user_id),
            }),
        }
    }

    #[ic_cdk::update]
    fn update_user_workout_plan(user_id: u64, payload: WorkoutPlanPayload) -> Result<WorkoutPlan, Error> {
        match WORKOUT_PLAN_STORAGE.with(|service| service.borrow().get(&user_id)) {
            Some(mut work_p) => {
                work_p.user_id = payload.user_id;
                work_p.push_ups = payload.push_ups;
                work_p.sit_ups = payload.sit_ups;
                work_p.running_time = payload.running_time;
                work_p.updated_at = Some(time());
                do_insert_wp(&work_p);
                Ok(work_p)
            },
            None => Err(Error::ServerError {
                msg: format!(
                    "couldn't update workplan for user with id={}",
                    user_id
                ),
            }),
        }
    }

    #[ic_cdk::update]
    fn delete_user_workout_plan(user_id: u64) -> Result<WorkoutPlan, Error> {
        match _get_workout(&user_id) {
            Some((_i, wp)) => {
                match WORKOUT_PLAN_STORAGE.with(|service| service.borrow_mut().remove(&wp.id)) {
                    Some(workout_plan) => Ok(workout_plan),
                    None => Err(Error::ServerError {
                        msg: format!(
                            "couldn't delete workplan for user with id={}.",
                            user_id
                        ),
                    }),
                }
            },
            None => Err(Error::NotFound {
                msg: format!(
                    "couldn't find workplan for user with id={}.",
                    user_id
                ),
            }),
        }
        
    }

    // helper method to perform workout plan insert.
    fn do_insert_wp(wp: &WorkoutPlan) {
        WORKOUT_PLAN_STORAGE.with(|service| service.borrow_mut().insert(wp.id, wp.clone()));
    }

    // Check if user workplan exists
    fn check_user_wp(user_id: u64) ->  bool {
        match _get_workout(&user_id) {
            Some((_i, _wp)) => true,
            None => false
        }
    }

    // a struct to cache workout plan data
    struct Cache<T>
        where T: Fn(u64) -> Option<WorkoutPlanPayload> {
            generate_workout: T,
            workout_plan: Option<WorkoutPlanPayload>,
        }

    impl<T> Cache<T>
        where T: Fn(u64) -> Option<WorkoutPlanPayload>

        {
            fn new(generate_workout: T) -> Cache<T> {
                Cache {
                    generate_workout,
                    workout_plan: None,
                }
            }

            fn workout_plan(&mut self, user_id: u64) -> Option<WorkoutPlanPayload> {
                match &self.workout_plan {
                    Some(v) => Some(v.clone()),
                    None => {
                        match _get_workout(&user_id) {
                            Some((_i, wp)) => {
                                self.workout_plan = Some(WorkoutPlanPayload {
                                    user_id: user_id,
                                    sit_ups: wp.sit_ups,
                                    push_ups: wp.push_ups,
                                    running_time: wp.running_time,
                                });
                                self.workout_plan.clone()
                            },
                            None => {
                                let wp = (self.generate_workout)(user_id);
                                self.workout_plan = wp.clone();
                                wp
                            }
                        }
                    }
                }
            }
        }

    
        fn _gen_wp(_user_id: &u64) -> Cache<impl Fn(u64) -> Option<WorkoutPlanPayload>> {
            let calculate_wp = Cache::new(|_user_id| {
                let user = _get_user(&_user_id);
                match user {
                    Some(user) => {
                        if user.age > 55 {
                            Some(WorkoutPlanPayload {
                                user_id: user.id,
                                push_ups: 5,
                                sit_ups: 10,
                                running_time: 20,
                            })
                        } else if user.height > 5 && user.weight < 80 {
                            Some(WorkoutPlanPayload {
                                user_id: user.id,
                                push_ups: 20,
                                sit_ups: 20,
                                running_time: 60,
                            })
                        } else {
                            Some(WorkoutPlanPayload {
                                user_id: user.id,
                                push_ups: 10,
                                sit_ups: 10,
                                running_time: 30,
                            })
                        }
                    },
                    None => None
                }
                
            });
            calculate_wp
        }

    // a helper method to get users workout plan
    fn _get_workout(user_id: &u64) -> Option<(u64, WorkoutPlan)> {
        WORKOUT_PLAN_STORAGE.with(|service| service.borrow().iter().find(|(_i, wp)| wp.user_id == *user_id))
    }
    
    #[derive(candid::CandidType, Deserialize, Serialize)]
    enum Error {
        NotFound { msg: String },
        Exists { msg: String },
        ServerError { msg: String }
    }
    
    // a helper method to get a user by id. used in get_user/update_user
    fn _get_user(id: &u64) -> Option<User> {
        USER_STORAGE.with(|service| service.borrow().get(id))
    }
    
    // need this to generate candid
    ic_cdk::export_candid!();