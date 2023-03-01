interface props {
  /**
   * this is the message to display
   */
  message: string;
}
/**
 * this is boutton comp
 * @param props t
 * @returns
 */
export default function Button(props: props) {
  return <button>{props.message}</button>; //
}
