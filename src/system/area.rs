use enum_derive::ParseEnumError;
use super::query_metadata::QueryMetadata;
use super::utils::Utils;
use super::fields::Fields;
use super::types::{
    SystemOption,
    AreaOption as AreaOptionPlain,
};
use super::common_structs::{
    AreaInstance,
    AreaOption as AreaOptionCompleted,
};

pub struct AreaParser<'a> {
    query_metadata: &'a QueryMetadata,
}

impl<'a> AreaParser<'a> {
    pub fn new(query_metadata: &'a QueryMetadata) -> Self {
        AreaParser {
            query_metadata,
        }
    }
}

impl<'a> AreaParser<'a> {
    /**
     * Execute the query being intended to an area
     */
    pub fn execute(&self) -> Result<(), &'static str> {
        let attributes_string = self.query_metadata.attributes.as_ref().unwrap();
        
        // Get area name
        let space_index = attributes_string.find(' ').ok_or("Name of an area cannot be recognized")?;
        let area_name = &attributes_string[0..space_index];
        let area_instance = AreaInstance::new(area_name);
        
        let attributes_string = &attributes_string.to_lowercase()[space_index + 1..];
        let mut substring_positions: Vec<usize> = vec![];
        
        // Find options
        for option in ["restriction", "fields", "index"] { // @todo: get from the config
            if let Some(index) = attributes_string.find(option) {
                substring_positions.push(index);
            }
        }
        if substring_positions.is_empty() {
            return Err("The query have no any options");
        }
        substring_positions.sort();
        let last_position = *substring_positions.last().unwrap();

        // Iterate through positions
        let iter = substring_positions
            .iter()
            .enumerate()
            .map(|(index, element)| (index, *element as usize));

        let mut options: Vec<AreaOptionPlain> = vec![];
        for (index, position) in iter {
            let space_index = attributes_string[position..].find(' ').ok_or("?")?;
            let option_name = &attributes_string[position..space_index + position];

            let components_string = if position == last_position {
                let range = space_index + position..;
                &attributes_string[range]
            } else {
                let range = space_index + position..substring_positions[index + 1];
                &attributes_string[range]
            };
            options.push(AreaOptionPlain {
                name: option_name,
                components_string: components_string.trim(),
            });
        }
        self.parse_options(options, area_instance)
    }
}

impl<'a> AreaParser<'a> {
    /**
     * Parse options of the query
     */
    fn parse_options(
        &self,
        options: Vec<AreaOptionPlain<'a>>,
        mut area_instance: AreaInstance<'a>,
    ) -> Result<(), &'static str> {
        let mut system_options: Vec<AreaOptionCompleted> = vec![];

        for option in options {
            let parsed_result: Result<SystemOption, ParseEnumError> = Utils
                ::capitalize_first_letter(option.name)
                .parse();
            let system_option = parsed_result.unwrap();
            
            let execution_result: Result<(), &str> = match system_option {
                SystemOption::Fields => {
                    let mut fields = Fields::new();
                    fields.recognize(option.components_string)?;
                    let area_option = AreaOptionCompleted
                        ::new(SystemOption::Fields, fields.instances);

                    system_options.push(area_option);
                    Ok(())
                },
                SystemOption::Restriction => Ok(()),
                SystemOption::Index => Ok(()),
            };
            execution_result?;
        }
        area_instance.options = system_options;
        area_instance.save()
    }
}
