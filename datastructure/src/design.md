                     ┌───────────┐
requests ──────────> │ Scheduler │
                     └─────┬─────┘
                           │
               ┌───────────┴───────────┐
               ↓                       ↓
        BinaryHeap                 VecDeque
        urgent work                normal work
               │                       │
               └───────────┬───────────┘
                           ↓
                    worker channel
                           ↓
                ┌──────────┼──────────┐
                ↓          ↓          ↓
             worker1    worker2    worker3