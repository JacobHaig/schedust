use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,

    pub task: Task,

    pub agent: String,
}
