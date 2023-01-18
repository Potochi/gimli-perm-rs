
# Intro

---
### What is Gimli?

Gimli is a 384-bit permutation designed to achieve high security with high performance across a broad range of platforms

---
 
### How?

- Trivially parallelizable
- Low memory usage
- Simple instructions (no multiplies/divides)
- Simple to implement

---

# Algorhitm

---

### The state
![[Pasted image 20230119103155.png#invert|600]]

---

### The non-linear layer

![[Pasted image 20230119103324.png#invert|600]]

---

### The linear layer

![[Pasted image 20230119103355.png#invert|600]]

--- 

# Code analysis

---

### The round function

```c
    for (column = 0; column < 4; ++column)
    {
      x = rotate(state[    column], 24);
      y = rotate(state[4 + column],  9);
      z =        state[8 + column];

      state[8 + column] = x ^ (z << 1) ^ ((y&z) << 2);
      state[4 + column] = y ^ x        ^ ((x|z) << 1);
      state[column]     = z ^ y        ^ ((x&y) << 3);
    }
```

- No data dependencies between columns
- Low variable usage
- Alignerd reads

---

### Compiler Explorer (rotations)

```c
	// GCC Trunk with -O3
	movdqu  xmm1, XMMWORD PTR [rdi]
	movdqa  xmm0, xmm1
	psrld   xmm1, 8
	pslld   xmm0, 24
	por     xmm0, xmm1
	movdqu  xmm1, XMMWORD PTR [rdi+16]
	movdqa  xmm2, xmm1
	psrld   xmm1, 23
	pslld   xmm2, 9
	por     xmm2, xmm1
	movdqu  xmm1, XMMWORD PTR [rdi+32]
```

* Gets vectorized by the compiler (this also removes the loop)

---

### UOps Analysis (troughput)

```c
	movdqu  xmm1, XMMWORD PTR [rdi]
	movdqa  xmm0, xmm1
	psrld   xmm1, 8
	pslld   xmm0, 24
	por     xmm0, xmm1
	movdqu  xmm1, XMMWORD PTR [rdi+16]
	movdqa  xmm2, xmm1
	psrld   xmm1, 23
	pslld   xmm2, 9
	por     xmm2, xmm1
	movdqu  xmm1, XMMWORD PTR [rdi+32]
	movdqa  xmm3, xmm1
	movdqa  xmm4, xmm1
	pand    xmm3, xmm2
	pslld   xmm4, 1
	pxor    xmm4, xmm0
	pslld   xmm3, 2
	pxor    xmm3, xmm4
	movdqa  xmm4, xmm2
	movups  XMMWORD PTR [rdi+32], xmm3
	movdqa  xmm3, xmm1
	pxor    xmm4, xmm0
	pxor    xmm1, xmm2
	por     xmm3, xmm0
	pand    xmm0, xmm2
	pslld   xmm3, 1
	pslld   xmm0, 3
	pxor    xmm3, xmm4
	pxor    xmm0, xmm1
	movups  XMMWORD PTR [rdi+16], xmm3
	movups  XMMWORD PTR [rdi], xmm0
	ret
```

---

### UOps Troughput Analysis (cycles/iteration)

| Tool | Skylake | Cascade Lake | Tiger Lake | Haswell |
| ---- | ------- | ------------ | ---------- | ------- |
| uiCA | 14.07   | 14.00        | 13.69      | 15.27   |

---

### Claimed performance metrics

The paper claims a `4.46 Cycle/Byte` throughput on Haswell, but does not mention the measuring methodology


---

### Comparison (Reference implementation)

* Compiled with CMAKE in Release mode using MinGW
* Execution time on 600MB file: `2.4344095s`

---

### Comparison (Rust Naive Implementation)

* Compiled with Cargo in Release mode
* Execution time on 600MB file:  `2.216268s`

--- 

### Comparison (Rust Simd Implementation)

* Compiled with Cargo in Release mode
* Execution time on 600MB file:  `2.1871634s`

---

### Possible reasons for slow SIMD

* Rust Portable Simd was used instead of writing the assembly by hand
* Reading the file has a high impact on the execution time

---

### Industry usage

* https://github.com/jedisct1/libhydrogen

---

### Third party benchmarks

| x86-64      | AEAD | Hash | Pw Hash | Key Exch | Sig  | Check |
| ----------- | ---- | ---- | ------- | -------- | ---- | ----- |
| Monocipher  | 307  | 683  | 511     | 8100     | 14K  | 6000  |
| libsodium   | 1000 | 870  | 701     | 21K      | 33K  | 13K   |
| TweetNaCL   | 51   | 40   |         | 1800     | 650  | 330   |
| LibHydrogen | 94   | 162  |         |          | 9200 | 5500  | 

Source: https://monocypher.org/speed

---

### LibHydrogen Speed

| Algorhitm         | Speed (X/s)     |
| ----------------- | --------------- |
| Auth'd encryption | 94 MB           |
| Hash              | 162 MB          |
| Sign              | 9233 signatures |
| Check             | 5513 checks     |
| Random            | 200 MB          | 


### Tools used
* https://uica.uops.info
* https://godbolt.org

---

# Thank you!

---