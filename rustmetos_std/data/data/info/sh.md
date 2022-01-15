# `sh`

Shell for short. Just invoke it as is and it will lead you
to an interactive prompt that can be used to do further
more powerfull tasks that can be done in core shell.

To invoke it within the core shell:

```
exec sh
```

## Switch to core shell from `sh`
```
boot set internal:core_shell
```
## Switch to `sh` from core shell
```
exec boot set sh
```