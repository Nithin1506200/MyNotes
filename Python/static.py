class Node :
    def __init__(self,data=None) -> None:
            self.data=data
            self.next : Node = None

class LinkedList:
    def __init__(self,data=None) -> None:
        self.head : Node= Node(data)
    def pushFront(self,data):
        new_node= Node(data)
        new_node.next=self.head
        self.head=new_node
    def append(self,data) -> None :
        new_node=Node(data)
        if self.head is None:
            self.head=new_node
        else:
            last=self.head
            while(last.next): #never let it point null
                last=last.next
            last.next=new_node
    def display(self):
        ptr=self.head
        while(ptr):
            print(ptr.data,end="->")
            ptr=ptr.next
        print("null")
    def isLooped(self)-> bool : # flyod technique
        slow=self.head
        fast=self.head
        while(slow and fast and fast.next):
            slow = slow.next
            fast=fast.next.next
            if slow == fast:
                return True
        return False
    def findLengthOfLoop(self) -> int : # t=O(n) s=O(1)
        slow = self.head
        fast = self.head
        flag=0
        while(slow and fast and slow.next and fast.next and fast.next.next):
            if (slow == fast and flag):
                count=1
                slow=slow.next
                while slow!=fast:
                    slow=slow.next
                    count+=1
                return count
            slow=slow.next
            fast=fast.next.next
            flag=1
        return 0

    def reverse(self) -> None:
        pass

# test for loop
ll:LinkedList = LinkedList(1)
ll.append(2)
ll.append(3)
ll.append(4)
ll.append(5)
ll.append(6)
ll.append(8)
ll.display()
if ll.isLooped() :
    print("it is looped")
