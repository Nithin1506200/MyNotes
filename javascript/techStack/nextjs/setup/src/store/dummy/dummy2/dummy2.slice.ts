
import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import sharedActions from "../../sharedActions/shared_actions";
interface DummyState {
  value: string;
}
const initialState: DummyState = {
  value: "dummy value",
};

export const dummySlice2 = createSlice({
  name: "dummy/dummy1",
  initialState,
  reducers: {
    changeName: (state, action: PayloadAction<string>) => {
      state.value = action.payload;
    },
  },
  extraReducers: (builder) => {
    builder.addCase(sharedActions.resetAllState, (state, action) => {
      // not valid
      //  state=initialState
      return initialState;
    });
  },
});
export const { changeName } = dummySlice2.actions;