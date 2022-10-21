import threading
import time
list={}
threads=[]
def calc(x):
  temp=x**3
  list[x]=temp
  
length=1000000
links=["link","link2"]


def calc_split(i,j):
    for k in range(i,j):
        calc(k)
tic=time.time()

t1=threading.Thread(target=calc_split, args=(0,length//2,))
t2=threading.Thread(target=calc_split, args=(length//2,length,))
t1.start()
t2.start()
t1.join()
t2.join()
print("multithreading:",time.time()-tic)
print(list)


list={}
tic=time.time()

calc_split(0,length)
print("singlethreading:",time.time()-tic)
