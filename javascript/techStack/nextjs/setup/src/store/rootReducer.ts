import { combineReducers } from "@reduxjs/toolkit";

import dummyReducer from "./dummy/dummy.combinedReducer";
// import storage from "redux-persist/lib/storage";
export const rootReducer = combineReducers({
  dummyReducer,
});
