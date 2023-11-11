use crate::muxml_types::{Part, PartListElement, ScorePart, Measure};
use crate::error::{Error, Result};

pub struct CompleteParts {
    part_list: PartListElement,
    part_elements: Vec<Part>,
}

impl CompleteParts {
    pub fn part_list(&self) -> &PartListElement {
        &self.part_list
    }
    pub fn part_elements(&self) -> &Vec<Part> {
        &self.part_elements
    }
    pub fn remove_part(&mut self, id: &str) -> Result<()> {
        let mut removed_count = 0u32;
        self.part_list.score_part.retain(|part| if part.id != id {
            true
        } else {
            removed_count += 1;
            false
        });
        // remove corresponding parts from the part_elements Vec, but don't count them
        self.part_elements.retain(|part| part.id != id);
        if removed_count != 0 {
            Ok(())
        } else {
            Err(Error::ItemNotFoundError(id.to_string()))
        }
    }
    pub fn new_part(&mut self, id: &str, part_name: &str) -> Result<()> {
        // Check for duplicate id
        let found = self.part_list.score_part.iter().find(|&item| item.id == id);
        if let Some(_) = found {
            Err(Error::DuplicateIndexError)
        } else {
            // No item matched this id, create a new one and indicate in result
            self.part_list.score_part.push(ScorePart {
                id: id.to_string(),
                part_name: part_name.to_string(),
            });
            Ok(())
        }
    }
    pub fn push_measure(&mut self, id: &str, measure: Measure) -> Result<()>  {
        // Check for duplicate id
        let found = self.part_elements.iter_mut().find(|item| item.id == id);
        if let Some(item) = found {
            item.measure.push(measure);
            Ok(())
        } else {
            // No item matched this id
            Err(Error::ItemNotFoundError(id.to_string()))
        }
    }
    pub fn extend_measures(&mut self, id: &str, measures: Vec<Measure>) -> Result<()>  {
        // Check for duplicate id
        let found = self.part_elements.iter_mut().find(|item| item.id == id);
        if let Some(item) = found {
            item.measure.extend_from_slice(measures.as_slice());
            Ok(())
        } else {
            // No item matched this id
            Err(Error::ItemNotFoundError(id.to_string()))
        }
    }
}

impl Default for CompleteParts {
    fn default() -> Self {
        Self {
            part_list: PartListElement { score_part: vec![] },
            part_elements: vec![],
        }
    }
}

//#[derive(Debug)]
pub struct Score {
    work_title: String,
    composer: String,
    software: String,
    encoding_date: String,
    complete_parts: CompleteParts,
    // Fields with defaults that cannot be overriden
    version: String,
}

impl Score {
    pub fn builder() -> ScoreBuilder {
        ScoreBuilder::default()
    }
    pub fn work_title(&self) -> &str {
        &self.work_title
    }
    pub fn composer(&self) -> &str {
        &self.composer
    }
    pub fn software(&self) -> &str {
        &self.software
    }
    pub fn encoding_date(&self) -> &str {
        &self.encoding_date
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn complete_parts(&self) -> &CompleteParts {
        &self.complete_parts
    }
}

impl Default for Score {
    fn default() -> Self {
        Self {
            version: "4.0".to_string(),
            work_title: "Untitled".to_string(),
            composer: "Composer / Arranger".to_string(),
            software: "muxml rust crate".to_string(),
            encoding_date: "2023-01-01".to_string(),
            complete_parts: CompleteParts::default(),
        }
    }
}

#[derive(Default)]
pub struct ScoreBuilder {
    work_title: String,
    composer: String,
    software: String,
    encoding_date: String,
    complete_parts: CompleteParts,
}

impl ScoreBuilder {
    pub fn new() -> Self {
        Self {
            work_title: "Untitled".to_string(),
            composer: "Composer / Arranger".to_string(),
            software: "muxml rust crate".to_string(),
            encoding_date: "2023-01-01".to_string(),
            complete_parts: CompleteParts::default(),
        }
    }
    pub fn work_title<S: Into<String>>(mut self, title: S) -> Self {
        self.work_title = title.into();
        self
    }

    pub fn composer<S: Into<String>>(mut self, composer: S) -> Self {
        self.composer = composer.into();
        self
    }

    pub fn software<S: Into<String>>(mut self, software: S) -> Self {
        self.software = software.into();
        self
    }

    pub fn encoding_date<S: Into<String>>(mut self, encoding_date: S) -> Self {
        self.encoding_date = encoding_date.into();
        self
    }

    pub fn complete_parts(mut self, complete_parts: CompleteParts) -> Self {
        self.complete_parts = complete_parts;
        self
    }

    pub fn build(self) -> Score {
        Score {
            work_title: self.work_title,
            composer: self.composer,
            software: self.software,
            encoding_date: self.encoding_date,
            complete_parts: self.complete_parts,
            ..Score::default()
        }
    }
}
