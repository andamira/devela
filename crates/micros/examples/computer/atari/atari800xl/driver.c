/* LLVM-MOS SDK supplies Atari startup and .xex packaging. */
extern void rust_atari_main(void);

int main(void) {
    rust_atari_main();
    for (;;) {
        __asm__ volatile("nop");
    }
}
