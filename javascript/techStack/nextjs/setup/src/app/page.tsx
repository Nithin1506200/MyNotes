import Image from "next/image";
import { Inter } from "next/font/google";
import styles from "./page.module.css";
import Link from "next/link";
import React, { useEffect, useState } from "react";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
  console.log("xxxx");

  return (
    <main>
      {/* <button onClick={clicked}>clikme</button> */}
      <div data-testid="mallika">Home page</div>

      <Link href={"./login"}> Login</Link>
    </main>
  );
}
