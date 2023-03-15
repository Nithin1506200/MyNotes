"use client";

import { useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { changeName } from "@/store/dummy/dummy1/dummy1.slice";
import { useAppDispatch, useAppSelector } from "hooks/store.hooks";
export default function Web() {
  const state = useAppSelector((store) => store.dummyReducer);
  console.log(state);
  console.log("testing", changeName.toString());
  const dispatch = useAppDispatch();
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
