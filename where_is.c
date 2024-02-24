#include <stdio.h>
#include <windows.h>

int main() {
  HMODULE k32 = GetModuleHandleA("kernel32.dll");
  printf("Kernel32 %#016x\n", k32);
  printf("WinExec %#016x\n", GetProcAddress(k32, "WinExec"));
}
