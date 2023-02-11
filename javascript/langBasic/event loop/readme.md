Event loop

- [javascript](#javascript)
  - [api](#api)
  - [Heap](#heap)
  - [Stack](#stack)
  - [Asynchronous callbacks](#asynchronous-callbacks)
  - [Call stack](#call-stack)
  - [Event queue](#event-queue)
  - [Task queue](#task-queue)
  - [The Event Loop](#the-event-loop)

## javascript

javascript is a single threaded, non blocking , asynchronous concurrent language

### api

- call stack
- event loop
- callback queue
- settimeout
- fetch
- etc

---

### Heap

memeory allocation

---

### Stack

holds the execution context

---

### Asynchronous callbacks

javascript has main thread which can do one thing at a time but has asynchronous callback for non blocking behaviour

### Call stack

it is responsible for keeping track of all the operations in the line to be executed, whenever a function is finished it is popped from the stack

### Event queue

responsible for sending new function to the stack for processing.

### Task queue

javascript can do only one thing at a time, the rest are queued to the task queue waiting to be executed

### The Event Loop

javascript has a runtime model based on an event loop, which is responsible for executing the code, collecting and processing events, and executing queued sub task

the event loop pushes the task from the task queue to the call stack
