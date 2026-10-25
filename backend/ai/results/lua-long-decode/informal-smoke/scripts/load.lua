local f=load("return 1+...") print(f(41)) print(load("syntax error here")) print(load("x=", "=chunk"))
