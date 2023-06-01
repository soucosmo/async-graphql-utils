use mongodb::bson::{Document, doc};
use async_graphql::SelectionField;


pub fn mount_project(fields: &SelectionField, project: &mut Document, father_field: Option<&str>) {
    let c = fields.selection_set();

    let mut id_exists: bool = false;

    let mut child_project: Document = doc! {};

    for i in c {
        let field: &str = i.name();

        if father_field.is_none() {
            if field == "id" {
                id_exists = true;
            } else {
                project.insert(field, 1);
            }
        } else {
            child_project.insert(field, 1);
        }

        for _ in i.selection_set() {
            mount_project(&i, project, Some(field));
            break;
        }
    }

    if !child_project.is_empty() && father_field.is_some(){
        project.insert(father_field.unwrap(), child_project);
    }

    if id_exists {
        project.insert("_id", 1);
    } else {
        project.insert("_id", 0);
    }
}
