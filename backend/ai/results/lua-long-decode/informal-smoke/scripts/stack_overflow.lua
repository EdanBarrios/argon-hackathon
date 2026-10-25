local function r() return 1 + r() end print(pcall(r))
