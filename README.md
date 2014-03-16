# Moon

Something that has the goal of being used seriously, but more as an experiment than anything else. The experiment is to see if a Lisp-dialect could be used effectively as a replacement for SQL querying. Thus, you'd use Moon instead of good old SQL. Crazy? Perhaps.

===

```lisp
(select :users
  (and 
    (where (> :age 18)
    (where (= :subscribed true)))))
```

## Goals

The language should be super simple. It's more-or-less going to be used for short-running queries. Thus, the VM is going to be teared down consistently.

* Rules/Functions
* Keywords `:age`
* Boolean
* And, Or
* `=`, `+`, `-`, `>`, `<`

The runtime should also be fairly straightforward. The goal is to get LLVM's JIT engine integrated. Thus, Moon would be a JITed language. This would work perfectly as a query language. All the functions within moon could be implemented in any native language, including Rust. LLVM would provide this ability.

Things like a Garbage Collector might not be super useful considering the short-lived lifecycle of the queries, but it's fun to implement.