local function fib(n) if n<2 then return n end return fib(n-1)+fib(n-2) end print(fib(20)) local function f(n) if n==0 then return 0 end return 1+f(n-1) end print(f(10000))
