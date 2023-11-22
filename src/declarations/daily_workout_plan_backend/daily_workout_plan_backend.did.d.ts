import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'NotFound' : { 'msg' : string } } |
  { 'Exists' : { 'msg' : string } } |
  { 'ServerError' : { 'msg' : string } };
export type Result = { 'Ok' : User } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : WorkoutPlan } |
  { 'Err' : Error };
export interface User {
  'id' : bigint,
  'age' : bigint,
  'weight' : bigint,
  'height' : bigint,
  'updated_at' : [] | [bigint],
  'name' : string,
  'created_at' : bigint,
}
export interface UserPayload {
  'age' : bigint,
  'weight' : bigint,
  'height' : bigint,
  'name' : string,
}
export interface UserUpdatePayload {
  'age' : [] | [bigint],
  'weight' : [] | [bigint],
  'height' : [] | [bigint],
  'name' : [] | [string],
}
export interface WorkoutPlan {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'push_ups' : bigint,
  'created_at' : bigint,
  'user_id' : bigint,
  'sit_ups' : bigint,
  'running_time' : bigint,
}
export interface WorkoutPlanUpdatePayload {
  'push_ups' : [] | [bigint],
  'user_id' : [] | [bigint],
  'sit_ups' : [] | [bigint],
  'running_time' : [] | [bigint],
}
export interface _SERVICE {
  'add_user' : ActorMethod<[UserPayload], [] | [User]>,
  'delete_user' : ActorMethod<[bigint], Result>,
  'delete_user_workout_plan' : ActorMethod<[bigint], Result_1>,
  'generate_workout_plan' : ActorMethod<[bigint], Result_1>,
  'get_user' : ActorMethod<[bigint], Result>,
  'get_user_workout_plan' : ActorMethod<[bigint], Result_1>,
  'update_user' : ActorMethod<[bigint, UserUpdatePayload], Result>,
  'update_user_workout_plan' : ActorMethod<
    [bigint, WorkoutPlanUpdatePayload],
    Result_1
  >,
}
