// "use client";
// import { configureStore, createStore } from "@reduxjs/toolkit";

import { configureStore } from "@reduxjs/toolkit";
import { rootReducer } from "./rootReducer";
import { useDispatch } from "react-redux";
import {
  FLUSH,
  PAUSE,
  PERSIST,
  PURGE,
  REGISTER,
  REHYDRATE,
  persistReducer,
  persistStore,
} from "redux-persist";
// import storage from "redux-persist/lib/storage";
import createWebStorage from "redux-persist/lib/storage/createWebStorage";

const createNoopStorage = () => {
  return {
    getItem(_key: any) {
      return Promise.resolve(null);
    },
    setItem(_key: any, value: any) {
      return Promise.resolve(value);
    },
    removeItem(_key: any) {
      return Promise.resolve();
    },
  };
};

const storage =
  typeof window !== "undefined"
    ? createWebStorage("session")
    : createNoopStorage();

// // export default storage;

const persistConfig = {
  key: "root",
  storage: storage,
  // whitelist: ["dummyReducer"], // only navigation will be persisted
};
const persistedReducer = persistReducer(persistConfig, rootReducer);
const store = configureStore({
  reducer: persistedReducer,
  middleware: (getDefaultMiddleware) => {
    return getDefaultMiddleware({
      serializableCheck: {
        ignoredActions: [FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER],
        // ignoredActions: [REHYDRATE],
      },
    });
  },
  //   middleware: getDefaultMiddleware({
  //     serializableCheck: {
  //       ignoredActions: [FLUSH, REHYDRATE, PAUSE, PERSIST, PURGE, REGISTER],
  //     },
  //   }),
});

// // store.sagaTask = sagaMiddleware.run(rootSaga);
const persistor = persistStore(store);
export default store;
export { persistor };
export type RootState = ReturnType<typeof store.getState>;
// for useSelector it saves you the need to type state:RootState everytime
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch = () => useDispatch<AppDispatch>();

// import { createStore, applyMiddleware } from "redux";
// import { persistStore } from "redux-persist";

// // import rootReducer from "./reducer";

// const store: any = (initialState: any) => {
//   let store;

//   // const sagaMiddleware = createSagaMiddleware();

//   const isClient = typeof window !== "undefined";

//   if (isClient) {
//     const { persistReducer } = require("redux-persist");
//     const storage = require("redux-persist/lib/storage").default;

//     const persistConfig = {
//       key: "root",
//       storage,
//     };

//     store = createStore(
//       persistReducer(persistConfig, rootReducer),
//       initialState
//       // applyMiddleware(sagaMiddleware)
//     );

//     store.__PERSISTOR = persistStore(store);
//   } else {
//     store = createStore(
//       rootReducer,
//       initialState
//       // applyMiddleware(sagaMiddleware)
//     );
//   }

//   // store.sagaTask = sagaMiddleware.run(rootSaga);

//   return store;
// };

// export default store;
// export type RootState = ReturnType<typeof store.getState>;
