use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

type JobId = u64;

#[derive(Debug, Clone)]
struct Job {
    id: JobId,
    name: String,
    priority: u8,
}

#[derive(Debug, Eq, PartialEq)]
struct PriorityJob {
    id: JobId,
    priority: u8,
}

// BinaryHeap needs ordering.
//
// Higher priority should come out first.
impl Ord for PriorityJob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.id.cmp(&self.id))
    }
}

impl PartialOrd for PriorityJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct JobScheduler {
    // ID -> full job metadata
    jobs: HashMap<JobId, Job>,

    // Prevent submitting same job twice
    known_jobs: HashSet<JobId>,

    // Normal FIFO jobs
    normal_queue: VecDeque<JobId>,

    // Urgent jobs
    priority_queue: BinaryHeap<PriorityJob>,

    // Most recently processed jobs
    processing_stack: Vec<JobId>,

    // Audit/event history
    history: LinkedList<String>,
}

impl JobScheduler {
    fn new() -> Self {
        Self {
            jobs: HashMap::new(),
            known_jobs: HashSet::new(),
            normal_queue: VecDeque::new(),
            priority_queue: BinaryHeap::new(),
            processing_stack: Vec::new(),
            history: LinkedList::new(),
        }
    }

    fn submit(&mut self, job: Job) {
        let id = job.id;

        // HashSet gives fast deduplication.
        if !self.known_jobs.insert(id) {
            self.history
                .push_back(format!("Rejected duplicate job {id}"));

            return;
        }

        let priority = job.priority;
        let name = job.name.clone();

        // HashMap stores complete job metadata.
        self.jobs.insert(id, job);

        if priority >= 10 {
            // Important jobs go into priority queue.
            self.priority_queue.push(PriorityJob { id, priority });

            self.history
                .push_back(format!("Priority job submitted: {id} ({name})"));
        } else {
            // Normal jobs are FIFO.
            self.normal_queue.push_back(id);

            self.history
                .push_back(format!("Normal job submitted: {id} ({name})"));
        }
    }

    fn next_job(&mut self) -> Option<Job> {
        // Always process priority jobs first.
        let job_id = if let Some(priority_job) = self.priority_queue.pop() {
            priority_job.id
        } else {
            self.normal_queue.pop_front()?
        };

        let job = self.jobs.get(&job_id)?.clone();

        // Stack remembers processing order.
        self.processing_stack.push(job_id);

        self.history
            .push_back(format!("Processing job {} ({})", job.id, job.name));

        Some(job)
    }

    fn retry_last(&mut self) {
        let Some(job_id) = self.processing_stack.pop() else {
            println!("No job available to retry");
            return;
        };

        let Some(job) = self.jobs.get(&job_id) else {
            return;
        };

        if job.priority >= 10 {
            self.priority_queue.push(PriorityJob {
                id: job.id,
                priority: job.priority,
            });
        } else {
            self.normal_queue.push_front(job.id);
        }

        self.history
            .push_back(format!("Retry requested for job {job_id}"));
    }

    fn get_job(&self, id: JobId) -> Option<&Job> {
        self.jobs.get(&id)
    }

    fn print_history(&self) {
        println!("\n=== Job History ===");

        for event in &self.history {
            println!("{event}");
        }
    }
}

fn main() {
    let mut scheduler = JobScheduler::new();

    scheduler.submit(Job {
        id: 1,
        name: "Send welcome email".into(),
        priority: 1,
    });

    scheduler.submit(Job {
        id: 2,
        name: "Generate analytics report".into(),
        priority: 3,
    });

    scheduler.submit(Job {
        id: 3,
        name: "Process payment".into(),
        priority: 100,
    });

    scheduler.submit(Job {
        id: 4,
        name: "Security alert".into(),
        priority: 50,
    });

    // Duplicate.
    scheduler.submit(Job {
        id: 3,
        name: "Duplicate payment".into(),
        priority: 100,
    });

    println!("=== Processing ===");

    while let Some(job) = scheduler.next_job() {
        println!(
            "Processing ID={} name={} priority={}",
            job.id, job.name, job.priority
        );
    }

    scheduler.print_history();
}
