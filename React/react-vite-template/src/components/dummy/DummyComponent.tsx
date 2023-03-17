import { dummySvg } from '@/assets/generic';
export default function Dummy() {
  return (
    <>
      <h1>{import.meta.env.VITE_APP}</h1>
      <img src={dummySvg} alt="" />
    </>
  );
}
