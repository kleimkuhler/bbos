# bbos

- [ ] I do not know how a kernel _actually_ works

## Creating a bootable disk image
```
$ bootimage build --target x86_64-bbos.json
```

## Booting disk image (either of the following)
```
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-bbos/debug/bootimage-bbos.bin
```

```
$ bootimage run
```
