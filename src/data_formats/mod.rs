pub struct IR {
    id: String,
    node_type: String,
    name: String,
    file: String,

    code: String,

    calls: Vec<String>,
    uses_variables: Vec<String>,
    depends_on_types: Vec<String>,

    defined_in_class: Vec<String>,

    children: Vec<String>,

    used_by: Vec<String>,
}

impl IR {
    pub fn new() -> IR {
        Self {
            id: String::new(),
            node_type: String::new(),
            name: String::new(),
            file: String::new(),
            code: String::new(),
            calls: Vec::new(),
            uses_variables: Vec::new(),
            depends_on_types: Vec::new(),
            defined_in_class: Vec::new(),
            children: Vec::new(),
            used_by: Vec::new(),
        }
    }

    pub fn add_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn add_node_type(&mut self, node_type: String) {
        self.node_type = node_type;
    }
}
