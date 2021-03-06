use crate::clients::{CosmosUriBuilder, DatabaseClient, ResourceType};
use crate::collection::CollectionName;
use crate::collection::{Collection, IndexingPolicy, PartitionKey};
use crate::prelude::*;
use crate::responses::CreateCollectionResponse;
use crate::Offer;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateCollectionBuilder<
    'a,
    CUB,
    OfferSet,
    CollectionNameSet,
    IndexingPolicySet,
    PartitionKeySet,
> where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
    p_offer: PhantomData<OfferSet>,
    p_collection_name: PhantomData<CollectionNameSet>,
    p_indexing_policy: PhantomData<IndexingPolicySet>,
    p_partition_key: PhantomData<PartitionKeySet>,
    offer: Option<Offer>,
    collection_name: Option<&'a dyn CollectionName>,
    indexing_policy: Option<&'a IndexingPolicy>,
    partition_key: Option<&'a PartitionKey>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> CreateCollectionBuilder<'a, CUB, No, No, No, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        database_client: &'a DatabaseClient<'a, CUB>,
    ) -> CreateCollectionBuilder<'a, CUB, No, No, No, No> {
        CreateCollectionBuilder {
            database_client,
            p_offer: PhantomData {},
            offer: None,
            p_collection_name: PhantomData {},
            collection_name: None,
            p_indexing_policy: PhantomData {},
            indexing_policy: None,
            p_partition_key: PhantomData {},
            partition_key: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    DatabaseClientRequired<'a, CUB>
    for CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn database_client(&self) -> &'a DatabaseClient<'a, CUB> {
        self.database_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB, CollectionNameSet, IndexingPolicySet, PartitionKeySet> OfferRequired
    for CreateCollectionBuilder<'a, CUB, Yes, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn offer(&self) -> Offer {
        self.offer.unwrap()
    }
}

impl<'a, CUB, OfferSet, IndexingPolicySet, PartitionKeySet> CollectionNameRequired<'a>
    for CreateCollectionBuilder<'a, CUB, OfferSet, Yes, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_name(&self) -> &'a dyn CollectionName {
        self.collection_name.unwrap()
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, PartitionKeySet> IndexingPolicyRequired<'a>
    for CreateCollectionBuilder<'a, CUB, OfferSet, CollectionNameSet, Yes, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn indexing_policy(&self) -> &'a IndexingPolicy {
        self.indexing_policy.unwrap()
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet> PartitionKeyRequired<'a>
    for CreateCollectionBuilder<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, Yes>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_key(&self) -> &'a PartitionKey {
        self.partition_key.unwrap()
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> UserAgentOption<'a>
    for CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> ActivityIdOption<'a>
    for CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    ConsistencyLevelOption<'a>
    for CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level
    }
}

impl<'a, CUB, CollectionNameSet, IndexingPolicySet, PartitionKeySet> OfferSupport
    for CreateCollectionBuilder<'a, CUB, No, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
where
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateCollectionBuilder<
        'a,
        CUB,
        Yes,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    #[inline]
    fn with_offer(self, offer: Offer) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: Some(offer),
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, OfferSet, IndexingPolicySet, PartitionKeySet> CollectionNameSupport<'a>
    for CreateCollectionBuilder<'a, CUB, OfferSet, No, IndexingPolicySet, PartitionKeySet>
where
    OfferSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateCollectionBuilder<'a, CUB, OfferSet, Yes, IndexingPolicySet, PartitionKeySet>;

    #[inline]
    fn with_collection_name(self, collection_name: &'a dyn CollectionName) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: Some(collection_name),
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, PartitionKeySet> IndexingPolicySupport<'a>
    for CreateCollectionBuilder<'a, CUB, OfferSet, CollectionNameSet, No, PartitionKeySet>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateCollectionBuilder<'a, CUB, OfferSet, CollectionNameSet, Yes, PartitionKeySet>;

    #[inline]
    fn with_indexing_policy(self, indexing_policy: &'a IndexingPolicy) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: Some(indexing_policy),
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet> PartitionKeySupport<'a>
    for CreateCollectionBuilder<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, No>
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateCollectionBuilder<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, Yes>;

    #[inline]
    fn with_partition_key(self, partition_key: &'a PartitionKey) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: Some(partition_key),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> UserAgentSupport<'a>
    for CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet> ActivityIdSupport<'a>
    for CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    ConsistencyLevelSupport<'a>
    for CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        CreateCollectionBuilder {
            database_client: self.database_client,
            p_offer: PhantomData {},
            p_collection_name: PhantomData {},
            p_indexing_policy: PhantomData {},
            p_partition_key: PhantomData {},
            offer: self.offer,
            collection_name: self.collection_name,
            indexing_policy: self.indexing_policy,
            partition_key: self.partition_key,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable regardless
impl<'a, CUB, OfferSet, CollectionNameSet, IndexingPolicySet, PartitionKeySet>
    CreateCollectionBuilder<
        'a,
        CUB,
        OfferSet,
        CollectionNameSet,
        IndexingPolicySet,
        PartitionKeySet,
    >
where
    OfferSet: ToAssign,
    CollectionNameSet: ToAssign,
    IndexingPolicySet: ToAssign,
    PartitionKeySet: ToAssign,
    CUB: CosmosUriBuilder,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> CreateCollectionBuilder<'a, CUB, Yes, Yes, Yes, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateCollectionResponse, AzureError> {
        trace!("CreateCollectionBuilder::execute called");

        let mut req = self.database_client.main_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name().name()),
            hyper::Method::POST,
            ResourceType::Collections,
        );

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        // add trait headers
        let req = OfferRequired::add_header(self, req);
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let mut collection = Collection::new(
            self.collection_name().name(),
            self.indexing_policy().to_owned(),
        );
        collection.parition_key = self.partition_key().to_owned();

        let body = serde_json::to_string(&collection)?;
        debug!("body == {}", body);

        let req = req.body(hyper::Body::from(body))?;
        debug!("\nreq == {:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.database_client.hyper_client().request(req),
            StatusCode::CREATED,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
