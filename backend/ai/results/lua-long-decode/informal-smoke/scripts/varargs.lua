local function f(...) local t=table.pack(...) return t.n, select("#",...), select(-1, ...) end print(f(1,nil,3,nil)) print((f(1,2)))
