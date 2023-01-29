class Node {
  constructor(value = undefined) {
    this.value = value;
    this.next = undefined;
  }
}
class LinkedList {
  //head is a node
  constructor(value) {
    this.head = new Node(value);
  }
  append(value) {
    let temp = this.head;
    while (temp.next) {
      temp = temp.next;
    }
    temp.next = new Node(value);
  }
  print() {
    let temp = this.head;
    while (temp) {
      console.log(temp.value);
      temp = temp.next;
    }
  }
}
function test() {
  const ll = new LinkedList(0);
  ll.append(3);
  ll.append(4);
  ll.print();
}
test();
