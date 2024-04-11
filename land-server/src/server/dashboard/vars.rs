use land_dao::{models, projects::ProjectCreatedBy, settings, DateTimeUTC};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProjectVar {
    pub id: i32,
    pub name: String,
    pub prod_domain: String,
    pub prod_domain_full: String,
    pub prod_domain_url: String,
    pub dev_domain: String,
    pub dev_domain_full: String,
    pub dev_domain_url: String,
    pub description: String,
    pub language: String,
    pub created_by: String,
    pub updated_at: DateTimeUTC,
    pub source: String,
    pub is_editable: bool,
}

impl ProjectVar {
    pub async fn from_models_vec(
        projects: Vec<models::project::Model>,
    ) -> anyhow::Result<Vec<ProjectVar>> {
        let (domain, protocol) = settings::get_domain_settings().await?;
        Ok(projects
            .into_iter()
            .map(|p| ProjectVar {
                id: p.id,
                name: p.name.clone(),
                prod_domain: p.prod_domain.clone(),
                prod_domain_full: format!("{}.{}", p.prod_domain, domain),
                prod_domain_url: format!("{}://{}.{}", protocol, p.prod_domain, domain),
                dev_domain: p.dev_domain.clone(),
                dev_domain_full: format!("{}.{}", p.dev_domain, domain),
                dev_domain_url: format!("{}://{}.{}", protocol, p.dev_domain, domain),
                language: p.language,
                is_editable: p.created_by == ProjectCreatedBy::Playground.to_string(),
                created_by: p.created_by,
                updated_at: p.updated_at.and_utc(),
                description: p.description,
                source: String::new(), // for list show, source is not needed
            })
            .collect())
    }
    pub async fn new(
        project: &land_dao::models::project::Model,
        playground: Option<&land_dao::models::playground::Model>,
    ) -> anyhow::Result<Self> {
        let (domain, protocol) = settings::get_domain_settings().await?;
        let mut var = ProjectVar {
            id: project.id,
            name: project.name.clone(),
            prod_domain: project.prod_domain.clone(),
            prod_domain_full: format!("{}.{}", project.prod_domain, domain),
            prod_domain_url: format!("{}://{}.{}", protocol, project.prod_domain, domain),
            dev_domain: project.dev_domain.clone(),
            dev_domain_full: format!("{}.{}", project.dev_domain, domain),
            dev_domain_url: format!("{}://{}.{}", protocol, project.dev_domain, domain),
            language: project.language.clone(),
            updated_at: project.updated_at.and_utc(),
            description: project.description.clone(),
            source: String::new(),
            created_by: project.created_by.clone(),
            is_editable: project.created_by == ProjectCreatedBy::Playground.to_string(),
        };
        if let Some(playground) = playground {
            var.source = playground.source.clone();
        }
        Ok(var)
    }
}