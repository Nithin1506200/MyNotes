interface props {
  /**
   * this is the message to display
   */
  message: string;
}
export default function Button(props: props) {
  return <button>{props.message}</button>; //
}
