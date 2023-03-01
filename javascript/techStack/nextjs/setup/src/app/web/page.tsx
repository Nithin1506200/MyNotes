"use client";
import { RootState } from "@/store/store";
import { useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { changeName } from "@/store/dummy/dummy1/dummy1.slice";
export default function Web() {
  const state: any = useSelector<RootState>((store) => store.dummyReducer);
  console.log(state);
  console.log("testing");
  const dispatch = useDispatch();
  return (
    <main>
      welcome to appo n<h1>dummy Reducer1 : {state.dummy1.value as string}</h1>
      <h2> dummy Reducer2 : {state.dummy2.value as string}</h2>
      <button
        onClick={() => {
          dispatch(changeName("nithin"));
          console.log;
        }}
      >
        {" "}
        change name to nithin
      </button>
    </main>
  );
}
//
