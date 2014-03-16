# Moon

Something that has the goal of being used seriously, but more as an experiment than anything else. The experiment is to see if a Lisp-dialect could be used effectively as a replacement for SQL querying. Thus, you'd use Moon instead of good old SQL. Crazy? Perhaps.

===

```lisp
(select :users
  (and 
    (where (> :age 18)
    (where (= :subscribed true)))))
```