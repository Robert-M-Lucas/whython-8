nasm -f elf64 ./test.asm
gcc ./test.o
./a.out
echo $?
