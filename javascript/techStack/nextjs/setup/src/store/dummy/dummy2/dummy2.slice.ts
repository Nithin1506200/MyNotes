
import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import sharedActions from "../../sharedActions/shared_actions";
interface DummyState {
  value: string;
}
const initialState: DummyState = {
  value: "dummy value",
};

export const dummySlice2 = createSlice({
  name: "dummy/dummy2",
  initialState,
  reducers: {
    changeNameOfSlice2: (state, action: PayloadAction<string>) => {
      state.value = action.payload;
    },
  },
  extraReducers: (builder) => {
    builder.addCase(sharedActions.resetAllState, (state, action) => {
      return initialState;
    });
  },
});
export const { changeNameOfSlice2 } = dummySlice2.actions;