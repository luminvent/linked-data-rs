use rdf_types::dataset::{BTreeDataset, IndexedBTreeDataset};
use rdf_types::interpretation::VocabularyInterpretation;
use rdf_types::{IriBuf, Literal, LiteralType, Quad, Term};
use rdf_types::vocabulary::NoVocabulary;
use linked_data_next::LinkedDataDeserializePredicateObjects;
use linked_data_next_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[ld(prefix("ex" = "http://example.org/"))]
struct Foo {
  #[ld("ex:name")]
  name: String,
  #[ld("ex:bar")]
  bar: Bar,
  #[ld(flatten)]
  more: MoreFoo,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[ld(prefix("ex" = "http://example.org/"))]
#[ld(type = "http://example.org/Bar")]
struct Bar {
  #[ld(id)]
  id: IriBuf,

  #[ld("ex:value")]
  value: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[ld(prefix("ex" = "http://example.org/"))]
struct MoreFoo {
  #[ld("ex:email")]
  email: String,
}


fn main() {
  let expected = Foo {
    name: "John Smith".to_string(),
    bar: Bar {
      id: IriBuf::new("http://example.org/myBar".to_string()).unwrap(),
      value: 1,
    },
    more: MoreFoo {
      email: "john.smith@example.org".to_string(),
    },
  };

  let mut dataset = BTreeDataset::new();

  let subject = Term::iri(IriBuf::new("http://example.org/myFoo".to_string()).unwrap());

  {
    let predicate = Term::iri(IriBuf::new("http://example.org/name".to_string()).unwrap());
    let object = Term::Literal(Literal::new("John Smith".to_string(), LiteralType::Any(IriBuf::new("http://www.w3.org/2001/XMLSchema#string".to_string()).unwrap())));

    dataset.insert(
      Quad(subject.clone(), predicate, object, None)
    );
  }

  {
    let subject = Term::iri(IriBuf::new("http://example.org/myFoo".to_string()).unwrap());
    let predicate = Term::iri(IriBuf::new("http://example.org/bar".to_string()).unwrap());
    let object = Term::iri(IriBuf::new("http://example.org/myBar".to_string()).unwrap());

    dataset.insert(
      Quad(subject, predicate, object.clone(), None)
    );


    let subject = object;
    let predicate = Term::iri(IriBuf::new("http://example.org/value".to_string()).unwrap());
    let object = Term::Literal(Literal::new("1".to_string(), LiteralType::Any(IriBuf::new("http://www.w3.org/2001/XMLSchema#unsignedInt".to_string()).unwrap())));

    dataset.insert(
      Quad(subject, predicate, object, None)
    );
  }

  {
    let subject = Term::iri(IriBuf::new("http://example.org/myFoo".to_string()).unwrap());
    let predicate = Term::iri(IriBuf::new("http://example.org/email".to_string()).unwrap());
    let object = Term::Literal(Literal::new("john.smith@example.org".to_string(), LiteralType::Any(IriBuf::new("http://www.w3.org/2001/XMLSchema#string".to_string()).unwrap())));

    dataset.insert(
      Quad(subject, predicate, object, None)
    );
  }

  let dataset = IndexedBTreeDataset::from_non_indexed(dataset);

  let interpretation = VocabularyInterpretation::<NoVocabulary>::new();

  let value = Foo::deserialize_objects(rdf_types::vocabulary::no_vocabulary(), &interpretation, &dataset, None, &[subject]).unwrap();

  assert_eq!(value, expected);
}

