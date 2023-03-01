import { combineReducers } from "@reduxjs/toolkit";

import { dummySlice1 } from "./dummy1/dummy1.slice";
import { dummySlice2 } from "./dummy2/dummy2.slice";
const dummyReducer = combineReducers({
  dummy1: dummySlice1.reducer,
  dummy2: dummySlice2.reducer,
});
export default dummyReducer;
