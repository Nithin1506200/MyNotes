import Link from "next/link";

export default function Login() {
  return (
    <>
      <div>user name</div>
      <input type="text" />
      <div>password</div>
      <input type="password" />
      <Link href={"./web"}>
        <button type="submit">Login </button>
      </Link>
    </>
  );
}
