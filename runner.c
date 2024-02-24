#include <stdio.h>
#include <windows.h>

int main(int argc, char **argv) {
  char shellcode[] = {
      #include "sc.x"
  };

  void *exec = VirtualAlloc(
      0,
      sizeof shellcode,
      MEM_COMMIT,
      PAGE_EXECUTE_READWRITE
  );

  memcpy(exec, shellcode, sizeof shellcode);

  printf("Coucou :)\n");

//  printf("(R) PEB addr = %#016x\n", ((int(*)()) exec)());
//  printf("(C) PEB addr = %#016x\n", __readgsqword(0x60));

//  printf("K32 SC #1 = %#016x\n", ((HMODULE(*)()) exec)());
//  printf("K32 SC #2 = %#016x\n", ((HMODULE(*)()) exec)());
//  printf("K32 W32 = %#016x\n", GetModuleHandle("kernel32.dll"));
  
//  FARPROC a = 0;
//  a = ((FARPROC(*)())exec)();
//  printf("WinExec = %#016x\n", a);
//  printf("WinExec = %#016x\n", GetProcAddress(GetModuleHandle("kernel32.dll"), "WinExec"));
//  a("calc.exe", 0);
  
  ((void(*)()) exec)();
}
