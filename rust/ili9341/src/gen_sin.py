
import math;
print("pub const SIN_TABLE : [u8;51] = [")
for i in range(0,51):
    f=float(i)
    f*=math.pi
    f/=100.
    s=255.*(math.sin(f)+0.0)/1.0
    d=int(s)
    #print(str(i)+":"+str(d))
    print(str(d)+",", end="")
    if (i & 7) ==7:
        print("//"+str(i-7))
print("];")
