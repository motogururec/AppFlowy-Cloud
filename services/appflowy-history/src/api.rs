use crate::application::AppState;
use crate::biz::history::get_snapshots;
use collab_entity::CollabType;
use tonic::{Request, Response, Status};
use tonic_proto::history::history_server::History;
use tonic_proto::history::{HistoryState, SnapshotMeta, SnapshotRequest};

pub struct HistoryImpl {
  pub state: AppState,
}

/// The `History` trait is automatically generated by the Tonic framework based on the definitions
/// provided in the `history.proto` file.
///
/// ## Modifying RPC Methods
/// - To add new RPC methods or modify existing ones, you should edit the `history.proto` file located at:
///   `libs/tonic-proto/proto/history.proto`
/// - After updating the protobuf file, you need to regenerate the Rust source code to reflect these changes.
///
/// ## Regenerating Code
/// - Code regeneration is handled by the `build.rs` script in the Tonic framework, which processes `.proto` files.
/// - To trigger this script and regenerate the code, run `cargo build` in the `tonic-proto` project.
///   This action rebuilds all project dependencies and updates generated code accordingly.
///
#[tonic::async_trait]
impl History for HistoryImpl {
  async fn get_snapshots(
    &self,
    request: Request<SnapshotRequest>,
  ) -> Result<Response<tonic_proto::history::RepeatedSnapshotMeta>, Status> {
    let request = request.into_inner();
    let collab_type = CollabType::from(request.collab_type);
    let data = get_snapshots(&request.object_id, &collab_type, &self.state.pg_pool).await?;
    Ok(Response::new(data))
  }

  async fn get_in_memory_history(
    &self,
    request: Request<SnapshotRequest>,
  ) -> Result<Response<HistoryState>, Status> {
    let request = request.into_inner();
    let resp = self
      .state
      .open_collab_manager
      .get_in_memory_history(request)
      .await?;
    Ok(Response::new(resp))
  }

  async fn get_in_disk_history(
    &self,
    _request: Request<SnapshotMeta>,
  ) -> Result<Response<HistoryState>, Status> {
    todo!()
  }
}
