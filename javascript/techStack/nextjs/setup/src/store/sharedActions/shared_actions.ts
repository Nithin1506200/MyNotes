import { createAction } from "@reduxjs/toolkit";

const resetAllState = createAction("RESET_ALL_STATE");
const sharedActions = { resetAllState };
export default sharedActions;
