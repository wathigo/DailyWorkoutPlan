export const idlFactory = ({ IDL }) => {
  const UserPayload = IDL.Record({
    'age' : IDL.Nat64,
    'weight' : IDL.Nat64,
    'height' : IDL.Nat64,
    'name' : IDL.Text,
  });
  const User = IDL.Record({
    'id' : IDL.Nat64,
    'age' : IDL.Nat64,
    'weight' : IDL.Nat64,
    'height' : IDL.Nat64,
    'user_principal' : IDL.Principal,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'name' : IDL.Text,
    'created_at' : IDL.Nat64,
  });
  const Error = IDL.Variant({
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'Exists' : IDL.Record({ 'msg' : IDL.Text }),
    'ServerError' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : User, 'Err' : Error });
  const WorkoutPlan = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'push_ups' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'user_id' : IDL.Nat64,
    'sit_ups' : IDL.Nat64,
    'running_time' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : WorkoutPlan, 'Err' : Error });
  const UserUpdatePayload = IDL.Record({
    'age' : IDL.Opt(IDL.Nat64),
    'weight' : IDL.Opt(IDL.Nat64),
    'height' : IDL.Opt(IDL.Nat64),
    'name' : IDL.Opt(IDL.Text),
  });
  const WorkoutPlanUpdatePayload = IDL.Record({
    'push_ups' : IDL.Opt(IDL.Nat64),
    'user_id' : IDL.Opt(IDL.Nat64),
    'sit_ups' : IDL.Opt(IDL.Nat64),
    'running_time' : IDL.Opt(IDL.Nat64),
  });
  return IDL.Service({
    'add_user' : IDL.Func([UserPayload], [IDL.Opt(User)], []),
    'delete_user' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_user_workout_plan' : IDL.Func([IDL.Nat64], [Result_1], []),
    'generate_workout_plan' : IDL.Func([IDL.Nat64], [Result_1], []),
    'get_user' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_user_workout_plan' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'update_user' : IDL.Func([IDL.Nat64, UserUpdatePayload], [Result], []),
    'update_user_workout_plan' : IDL.Func(
        [IDL.Nat64, WorkoutPlanUpdatePayload],
        [Result_1],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
