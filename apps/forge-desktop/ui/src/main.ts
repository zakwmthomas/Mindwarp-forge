type KernelStatus = {
  mode: string;
  message: string;
  object_count: number;
  event_count: number;
  candidate_count: number;
};

type AtlasMilestone = { id: string; stream: string; name: string; status: string; depends_on: string[]; exit_criteria: string[] };
type ProjectAtlas = { project: { name: string; vision: string }; systems: { id: string; name: string; stream: string; purpose: string }[]; milestones: AtlasMilestone[] };
type OperatingSnapshot = { policy: { policies: { id: string; status: string; rule: string }[]; proposals: unknown[] }; active_checkpoint: { batch_id: string; state: string; substage_id: string; objective: string; atlas_route: { milestone: string; systems: string[] }; risk_level: string; research_gate: string; next_action: string }; catalogue_available: boolean };

type CodexCaptureStatus = {
  state: string;
  sessions: number;
  captured_messages: number;
  skipped_records: number;
  paused_sources: number;
  last_error: string | null;
};

type ImportReport = {
  source_id: string;
  message_count: number;
  candidate_count: number;
  correction_intents: number;
  approval_intents: number;
  already_recorded: boolean;
};

type DossierCandidate = {
  id: string;
  evidence_id: string;
  state: string;
  history_event_count: number;
};

type DossierSnapshot = {
  object_count: number;
  event_count: number;
  candidates: DossierCandidate[];
  applications: { event_id: string; relative_path: string; preimage_object: string | null; postimage_object: string | null; rolled_back: boolean }[];
};

type ReferenceStudioSnapshot = {
  projection_schema_version: number;
  requested_schema_version: number;
  compatibility: "compatible" | "version_mismatch";
  projection_source: string;
  verified_at_ms: number;
  read_only: boolean;
  limitations: string[];
  records: {
    work_packages: { id: string; stage: string; authority_lane: string; next_action: string }[];
    gate_receipts: { id: string; work_package_id: string; from_stage: string; to_stage: string; outcome: string; evidence_ids: string[]; failure_reason: string | null }[];
    blockers: { id: string; work_package_id: string; blocker_type: string; affected_stage: string; requirement: string; status: string }[];
    rollbacks: { id: string; work_package_id: string; gate_receipt_id: string; affected_artifact: string; reason: string }[];
    source_gaps: { id: string; location: string; availability: string; freshness: string; limitations: string }[];
    proof_receipts: {
      schema_version: number;
      receipt_id: string;
      system_id: string;
      proof_id: string;
      status: string;
      failure_classification: string | null;
      input_refs: string[];
      fixture_id: string;
      output_refs: string[];
      equivalence_method: string;
      limitations: string[];
    }[];
  };
};

type ReferenceViewportSnapshot = {
  schema_version: number;
  mode: string;
  renderer_profile: string;
  scene_fingerprint: string;
  read_only: boolean;
  edges: { from: string; to: string; role: string }[];
  frames: {
    index: number;
    views: { view: "front" | "side" | "top"; points: { id: string; x: number; y: number }[] }[];
  }[];
  limitations: string[];
};

type ControlledStimulusBundle = {
  schema_version: number;
  status: string;
  viewport_profile: string;
  base_scene_fingerprint: string;
  stimuli: { stimulus_id: string; control: "broken_connection" | "silhouette_collapse" | "articulation_drift" | null; snapshot: ReferenceViewportSnapshot }[];
  observed_claim_count: number;
  limitations: string[];
};

type OwnerObservationReceipt = {
  status: string;
  receipt_fingerprint: string;
  base_scene_fingerprint: string;
  compared_scene_fingerprint: string;
  pair_index: number;
  control: string;
  outcome: string;
  confidence: number;
  observed_claim_count: number;
  authority_effect: string;
  limitations: string[];
};

type BackupReceipt = {
  path: string;
  sha256: string;
  bytes: number;
  object_count: number;
  event_count: number;
  candidate_count: number;
};

type CodeAdmissionReceipt = {
  source_id: string;
  relative_path: string;
  language: string;
  code_object: string;
  manifest_object: string;
  candidate: string;
  already_recorded: boolean;
};

type OwnerBriefItem = {
  candidate_id: string;
  evidence_id: string;
  state: string;
};

type OwnerBrief = {
  pending_decision_count: number;
  visible_decisions: OwnerBriefItem[];
  truncated: boolean;
};

type TechnicalReference = { label: string; value: string };
type OwnerAction = {
  action_id: string;
  title: string;
  summary: string;
  why_now: string;
  estimated_minutes: number;
  can_defer: boolean;
  defer_effect: string;
  destination: string;
  progress_current: number;
  progress_total: number;
  technical_references: TechnicalReference[];
};
type OwnerDashboardSnapshot = {
  schema_version: number;
  actionable_count: number;
  primary_action: OwnerAction | null;
  later_action_count: number;
  reference: { knowledge_record_count: number; background_candidate_count: number; description: string };
  health: { label: string; summary: string; repository: string; integrity: string; authority: string };
};

type AuthorizationReceipt = { action: string; candidate_id: string; event_id: string };

type CodePreview = { candidate: string; relative_path: string; language: string; code: string; code_object: string };
type AppliedCodeReceipt = { candidate_id: string; path: string; event_id: string };
type WorkspaceFile = { relative_path: string; bytes: number; sha256: string };
type KnowledgeRecord = { id: string; record_type: string; state: string; title: string; summary: string; source_evidence_ids: string[]; source_actor: string; classifier_confidence: number; authority_lane: string };
type ForgeSnapshot = { schema_version: number; revision: string; read_only: boolean; master_program: { items: { id: string; status: string }[] }; active_checkpoint: { batch_id: string; next_action: string }; atlas: ProjectAtlas; knowledge_records: KnowledgeRecord[] };

const statusElement = document.querySelector<HTMLParagraphElement>("#status");
const knowledgeSummary = document.querySelector<HTMLParagraphElement>("#knowledge-summary");
const knowledgeList = document.querySelector<HTMLOListElement>("#knowledge-list");
const refreshKnowledge = document.querySelector<HTMLButtonElement>("#refresh-knowledge");
const knowledgeSearch = document.querySelector<HTMLInputElement>("#knowledge-search");
const knowledgeMore = document.querySelector<HTMLParagraphElement>("#knowledge-more");
const knowledgeFilters = Array.from(document.querySelectorAll<HTMLButtonElement>("[data-knowledge-filter]"));
let currentKnowledge: KnowledgeRecord[] = [];
let currentKnowledgeFilter = "none";
let currentKnowledgeSearch = "";
let currentOwnerAction: OwnerAction | null = null;
let currentReviewStep = 0;
let reviewPreviewPinned = false;
const mainElement = document.querySelector<HTMLElement>("main");
const goToReview = document.querySelector<HTMLButtonElement>("#go-to-review");
const toggleDetails = document.querySelector<HTMLButtonElement>("#toggle-details");
const referenceReview = document.querySelector<HTMLElement>("#reference-review");
const receiptElement = document.querySelector<HTMLParagraphElement>("#receipt");
const form = document.querySelector<HTMLFormElement>("#import-form");
const sourceInput = document.querySelector<HTMLInputElement>("#source-id");
const transcriptInput = document.querySelector<HTMLTextAreaElement>("#transcript");
const dossierSummary = document.querySelector<HTMLParagraphElement>("#dossier-summary");
const candidateList = document.querySelector<HTMLUListElement>("#candidate-list");
const candidateMore = document.querySelector<HTMLParagraphElement>("#candidate-more");
const refreshDossier = document.querySelector<HTMLButtonElement>("#refresh-dossier");
const refreshReferenceStudio = document.querySelector<HTMLButtonElement>("#refresh-reference-studio");
const referenceStudioSummary = document.querySelector<HTMLParagraphElement>("#reference-studio-summary");
const referenceStudioRecords = document.querySelector<HTMLUListElement>("#reference-studio-records");
const referenceStudioGaps = document.querySelector<HTMLUListElement>("#reference-studio-gaps");
const referenceViewportSummary = document.querySelector<HTMLParagraphElement>("#reference-viewport-summary");
const referenceViewportCanvas = document.querySelector<HTMLCanvasElement>("#reference-viewport-canvas");
const referenceViewportFrame = document.querySelector<HTMLInputElement>("#reference-viewport-frame");
const referenceViewportStimulus = document.querySelector<HTMLSelectElement>("#reference-viewport-stimulus");
const referenceViewportLimitations = document.querySelector<HTMLUListElement>("#reference-viewport-limitations");
const refreshReferenceViewport = document.querySelector<HTMLButtonElement>("#refresh-reference-viewport");
const referenceObservationForm = document.querySelector<HTMLFormElement>("#reference-observation-form");
const referenceObservationPair = document.querySelector<HTMLSelectElement>("#reference-observation-pair");
const referenceObservationOutcome = document.querySelector<HTMLSelectElement>("#reference-observation-outcome");
const referenceObservationConfidence = document.querySelector<HTMLSelectElement>("#reference-observation-confidence");
const referenceObservationReceipt = document.querySelector<HTMLParagraphElement>("#reference-observation-receipt");
const referenceObservationTechnical = document.querySelector<HTMLDetailsElement>("#reference-observation-technical");
const referenceObservationTechnicalText = document.querySelector<HTMLParagraphElement>("#reference-observation-technical-text");
const reviewStepLabel = document.querySelector<HTMLParagraphElement>("#review-step-label");
const reviewProgressFill = document.querySelector<HTMLElement>("#review-progress-fill");
const reviewCheckTitle = document.querySelector<HTMLHeadingElement>("#review-check-title");
const reviewCheckInstruction = document.querySelector<HTMLParagraphElement>("#review-check-instruction");
const showReference = document.querySelector<HTMLButtonElement>("#show-reference");
const showCurrentComparison = document.querySelector<HTMLButtonElement>("#show-current-comparison");
const reviewPreviousStep = document.querySelector<HTMLButtonElement>("#review-previous-step");
const reviewNextStep = document.querySelector<HTMLButtonElement>("#review-next-step");
const backupButton = document.querySelector<HTMLButtonElement>("#create-backup");
const backupReceipt = document.querySelector<HTMLParagraphElement>("#backup-receipt");
const codeForm = document.querySelector<HTMLFormElement>("#code-form");
const codeSource = document.querySelector<HTMLInputElement>("#code-source");
const codePath = document.querySelector<HTMLInputElement>("#code-path");
const codeLanguage = document.querySelector<HTMLInputElement>("#code-language");
const codeText = document.querySelector<HTMLTextAreaElement>("#code-text");
const codeReceipt = document.querySelector<HTMLParagraphElement>("#code-receipt");
const refreshBrief = document.querySelector<HTMLButtonElement>("#refresh-brief");
const briefSummary = document.querySelector<HTMLParagraphElement>("#brief-summary");
const briefList = document.querySelector<HTMLOListElement>("#brief-list");
const authorizationForm = document.querySelector<HTMLFormElement>("#authorization-form");
const authorizationAction = document.querySelector<HTMLSelectElement>("#authorization-action");
const authorizationCandidate = document.querySelector<HTMLInputElement>("#authorization-candidate");
const authorizationCorrection = document.querySelector<HTMLInputElement>("#authorization-correction");
const authorizationReplacement = document.querySelector<HTMLInputElement>("#authorization-replacement");
const authorizationHint = document.querySelector<HTMLParagraphElement>("#authorization-hint");
const authorizationConfirmation = document.querySelector<HTMLInputElement>("#authorization-confirmation");
const authorizationReceipt = document.querySelector<HTMLParagraphElement>("#authorization-receipt");
const previewCandidate = document.querySelector<HTMLInputElement>("#preview-candidate");
const previewButton = document.querySelector<HTMLButtonElement>("#preview-code");
const previewSummary = document.querySelector<HTMLParagraphElement>("#preview-summary");
const codePreview = document.querySelector<HTMLPreElement>("#code-preview");
const applyForm = document.querySelector<HTMLFormElement>("#apply-form");
const applyCandidate = document.querySelector<HTMLInputElement>("#apply-candidate");
const applyConfirmation = document.querySelector<HTMLInputElement>("#apply-confirmation");
const applyReceipt = document.querySelector<HTMLParagraphElement>("#apply-receipt");
const rollbackForm = document.querySelector<HTMLFormElement>("#rollback-form");
const rollbackEvent = document.querySelector<HTMLInputElement>("#rollback-event");
const rollbackConfirmation = document.querySelector<HTMLInputElement>("#rollback-confirmation");
const rollbackReceipt = document.querySelector<HTMLParagraphElement>("#rollback-receipt");
const workspaceButton = document.querySelector<HTMLButtonElement>("#refresh-workspace");
const workspaceSummary = document.querySelector<HTMLParagraphElement>("#workspace-summary");
const workspaceFiles = document.querySelector<HTMLUListElement>("#workspace-files");
const projectButton = document.querySelector<HTMLButtonElement>("#refresh-project");
const projectSummary = document.querySelector<HTMLParagraphElement>("#project-summary");
const atlasSummary = document.querySelector<HTMLParagraphElement>("#atlas-summary");
const atlasMilestones = document.querySelector<HTMLUListElement>("#atlas-milestones");
const atlasSystems = document.querySelector<HTMLUListElement>("#atlas-systems");
const refreshAtlas = document.querySelector<HTMLButtonElement>("#refresh-atlas");
const operatingSummary = document.querySelector<HTMLParagraphElement>("#operating-summary");
const operatingPolicies = document.querySelector<HTMLUListElement>("#operating-policies");
const operatingOwnerTitle = document.querySelector<HTMLHeadingElement>("#operating-owner-title");
const operatingOwnerSummary = document.querySelector<HTMLParagraphElement>("#operating-owner-summary");
const refreshOperating = document.querySelector<HTMLButtonElement>("#refresh-operating");
const codexCaptureSummary = document.querySelector<HTMLParagraphElement>("#codex-capture-status");
const captureOwnerState = document.querySelector<HTMLHeadingElement>("#capture-owner-state");
const captureOwnerSummary = document.querySelector<HTMLParagraphElement>("#capture-owner-summary");
const refreshCodexCapture = document.querySelector<HTMLButtonElement>("#refresh-codex-capture");
const pauseCodexCapture = document.querySelector<HTMLButtonElement>("#pause-codex-capture");
const resumeCodexCapture = document.querySelector<HTMLButtonElement>("#resume-codex-capture");
const homeHealth = document.querySelector<HTMLParagraphElement>("#home-health");
const homeAttention = document.querySelector<HTMLParagraphElement>("#home-attention");
const currentActionHeading = document.querySelector<HTMLHeadingElement>("#current-action-heading");
const homeActionSummary = document.querySelector<HTMLParagraphElement>("#home-action-summary");
const homeActionWhy = document.querySelector<HTMLParagraphElement>("#home-action-why");
const homeActionTime = document.querySelector<HTMLParagraphElement>("#home-action-time");
const homeActionProgress = document.querySelector<HTMLElement>("#home-action-progress");
const homeProgressFill = document.querySelector<HTMLElement>("#home-progress-fill");
const homeActionTechnical = document.querySelector<HTMLElement>("#home-action-technical");
const deferAction = document.querySelector<HTMLButtonElement>("#defer-action");
const deferEffect = document.querySelector<HTMLParagraphElement>("#defer-effect");
const workActionTitle = document.querySelector<HTMLHeadingElement>("#work-action-title");
const workActionSummary = document.querySelector<HTMLParagraphElement>("#work-action-summary");
const workActionWhy = document.querySelector<HTMLParagraphElement>("#work-action-why");
const workActionTechnical = document.querySelector<HTMLElement>("#work-action-technical");
const workOpenAction = document.querySelector<HTMLButtonElement>("#work-open-action");
const evidenceOwnerSummary = document.querySelector<HTMLParagraphElement>("#evidence-owner-summary");
const captureChip = document.querySelector<HTMLElement>("#capture-chip");
const workCount = document.querySelector<HTMLElement>("#work-count");
const navButtons = Array.from(document.querySelectorAll<HTMLButtonElement>("[data-nav]"));
const pages = Array.from(document.querySelectorAll<HTMLElement>("[data-page]"));

function navigate(pageName: string): void {
  const destination = pages.find((page) => page.dataset.page === pageName);
  if (!destination) return;
  pages.forEach((page) => page.classList.toggle("is-active", page === destination));
  navButtons.forEach((button) => {
    const selected = button.dataset.nav === pageName;
    button.classList.toggle("is-active", selected);
    if (selected) button.setAttribute("aria-current", "page");
    else button.removeAttribute("aria-current");
  });
  window.location.hash = pageName;
  document.querySelector<HTMLElement>("#main-content")?.focus({ preventScroll: true });
  window.scrollTo({ top: 0, behavior: "smooth" });
}

navButtons.forEach((button) => button.addEventListener("click", () => navigate(button.dataset.nav ?? "home")));
window.addEventListener("hashchange", () => navigate(window.location.hash.slice(1) || "home"));

async function invokeForge<T>(command: string, arguments_: Record<string, unknown> = {}): Promise<T> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(command, arguments_);
}

function renderTechnicalReferences(target: HTMLElement | null, references: TechnicalReference[]): void {
  if (!target) return;
  target.replaceChildren(...references.flatMap((reference) => {
    const term = document.createElement("dt");
    const value = document.createElement("dd");
    term.textContent = reference.label;
    value.textContent = reference.value;
    return [term, value];
  }));
}

function shorten(text: string, limit: number): string {
  const clean = text.replace(/\s+/g, " ").trim();
  return clean.length <= limit ? clean : `${clean.slice(0, limit - 1).trimEnd()}…`;
}

function friendlyRecordTitle(record: KnowledgeRecord): string {
  const withoutReceipt = record.title
    .replace(/^BOOTSTRAP RECEIPT\s*[—-]?\s*/i, "")
    .replace(/^Active objective:\s*/i, "");
  return shorten(withoutReceipt || `${record.record_type} record`, 110);
}

function renderKnowledge(): void {
  if (!knowledgeList) return;
  const filteredByType = currentKnowledgeFilter === "none"
    ? []
    : currentKnowledgeFilter === "all"
    ? currentKnowledge
    : currentKnowledge.filter((record) => record.record_type === currentKnowledgeFilter);
  const query = currentKnowledgeSearch.toLocaleLowerCase();
  const searchPool = query && currentKnowledgeFilter === "none" ? currentKnowledge : filteredByType;
  const matches = query
    ? searchPool.filter((record) => `${record.title} ${record.summary}`.toLocaleLowerCase().includes(query))
    : filteredByType;
  const visible = matches.slice(0, 30);
  knowledgeList.replaceChildren(...visible.map((record) => {
    const item = document.createElement("li");
    const heading = document.createElement("strong");
    heading.textContent = `${record.record_type.toUpperCase()} — ${friendlyRecordTitle(record)}`;
    const detail = document.createElement("p");
    detail.textContent = shorten(record.summary, 240);
    const technical = document.createElement("details");
    technical.className = "technical-details";
    const summary = document.createElement("summary");
    summary.textContent = "Technical details";
    const exact = document.createElement("p");
    exact.textContent = `Record ${record.id}; source ${record.source_actor}; state ${record.state}; authority ${record.authority_lane}; evidence links ${record.source_evidence_ids.length}.`;
    technical.append(summary, exact);
    item.append(heading, detail, technical);
    return item;
  }));
  if (knowledgeMore) knowledgeMore.textContent = currentKnowledgeFilter === "none" && !query
    ? "Search or choose a category to open the project library."
    : matches.length > visible.length
    ? `Showing the first ${visible.length} of ${matches.length} matches. Refine the search to narrow the library.`
    : `${matches.length} matching record${matches.length === 1 ? "" : "s"}.`;
}

async function loadKnowledge(): Promise<void> {
  if (!knowledgeSummary || !knowledgeList) return;
  try {
    const snapshot = await invokeForge<ForgeSnapshot>("forge_snapshot");
    currentKnowledge = snapshot.knowledge_records;
    knowledgeSummary.textContent = `${currentKnowledge.length} saved project records. They are reference material, not a list of things you need to approve.`;
    renderKnowledge();
  } catch (error) {
    knowledgeSummary.textContent = `Project knowledge is unavailable: ${String(error)}`;
    currentKnowledge = [];
    renderKnowledge();
  }
}

refreshKnowledge?.addEventListener("click", () => void loadKnowledge());
knowledgeFilters.forEach((button) => button.addEventListener("click", () => {
  currentKnowledgeFilter = button.dataset.knowledgeFilter ?? "all";
  knowledgeFilters.forEach((candidate) => candidate.setAttribute("aria-pressed", String(candidate === button)));
  renderKnowledge();
}));
knowledgeSearch?.addEventListener("input", () => {
  currentKnowledgeSearch = knowledgeSearch.value;
  renderKnowledge();
});

function openCurrentOwnerAction(): void {
  if (!currentOwnerAction) return;
  navigate(currentOwnerAction.destination);
  if (currentOwnerAction.destination === "studio") {
    referenceReview?.scrollIntoView({ behavior: "smooth", block: "start" });
  }
}

async function loadOwnerDashboard(): Promise<void> {
  try {
    const snapshot = await invokeForge<OwnerDashboardSnapshot>("owner_dashboard_snapshot");
    currentOwnerAction = snapshot.primary_action;
    if (workCount) {
      workCount.textContent = String(snapshot.actionable_count);
      workCount.title = `${snapshot.actionable_count} owner action${snapshot.actionable_count === 1 ? "" : "s"}`;
      workCount.hidden = snapshot.actionable_count === 0;
    }
    if (homeHealth) homeHealth.textContent = snapshot.health.summary;
    if (evidenceOwnerSummary) {
      evidenceOwnerSummary.textContent = `${snapshot.reference.knowledge_record_count} project records are stored locally. ${snapshot.reference.description}`;
    }
    const action = snapshot.primary_action;
    if (!action) {
      if (currentActionHeading) currentActionHeading.textContent = "Nothing needs you right now";
      if (homeActionSummary) homeActionSummary.textContent = "Forge has no unresolved owner decision or observation ready for review.";
      if (homeActionWhy) homeActionWhy.textContent = "Background work and reference material remain available without becoming owner tasks.";
      if (homeActionTime) homeActionTime.textContent = "";
      if (homeActionProgress) homeActionProgress.textContent = "CLEAR";
      if (homeProgressFill) homeProgressFill.style.width = "100%";
      if (homeAttention) homeAttention.textContent = "Your owner queue is clear.";
      if (goToReview) goToReview.hidden = true;
      if (deferAction) deferAction.hidden = true;
      if (workActionTitle) workActionTitle.textContent = "Nothing needs you right now";
      if (workActionSummary) workActionSummary.textContent = "Completed and background records are available in the project library below.";
      if (workActionWhy) workActionWhy.textContent = "Forge will raise a new card only when a deliberate owner action becomes dependency-ready.";
      if (workOpenAction) workOpenAction.hidden = true;
      renderTechnicalReferences(homeActionTechnical, []);
      renderTechnicalReferences(workActionTechnical, []);
      return;
    }
    const progress = action.progress_total ? Math.round((action.progress_current / action.progress_total) * 100) : 0;
    if (currentActionHeading) currentActionHeading.textContent = action.title;
    if (homeActionSummary) homeActionSummary.textContent = action.summary;
    if (homeActionWhy) homeActionWhy.textContent = action.why_now;
    if (homeActionTime) homeActionTime.textContent = `About ${action.estimated_minutes} minute${action.estimated_minutes === 1 ? "" : "s"}`;
    if (homeActionProgress) homeActionProgress.textContent = `${action.progress_current} OF ${action.progress_total}`;
    if (homeProgressFill) homeProgressFill.style.width = `${progress}%`;
    if (homeAttention) homeAttention.textContent = snapshot.later_action_count
      ? `One action needs you now; ${snapshot.later_action_count} can wait.`
      : "One short action needs you. Background project records do not require review.";
    if (deferEffect) deferEffect.textContent = action.defer_effect;
    if (goToReview) goToReview.hidden = false;
    if (deferAction) deferAction.hidden = !action.can_defer;
    renderTechnicalReferences(homeActionTechnical, action.technical_references);
    if (workActionTitle) workActionTitle.textContent = action.title;
    if (workActionSummary) workActionSummary.textContent = `${action.summary} About ${action.estimated_minutes} minutes.`;
    if (workActionWhy) workActionWhy.textContent = `${action.why_now} ${action.defer_effect}`;
    if (workOpenAction) workOpenAction.hidden = false;
    renderTechnicalReferences(workActionTechnical, action.technical_references);
  } catch (error) {
    if (homeAttention) homeAttention.textContent = "The owner queue could not be checked. Technical diagnostics remain available in Activity.";
    if (workActionTitle) workActionTitle.textContent = "Owner queue unavailable";
    if (workActionSummary) workActionSummary.textContent = "Forge could not build the read-only owner view.";
    if (workActionWhy) workActionWhy.textContent = String(error);
  }
}

deferAction?.addEventListener("click", () => {
  if (!currentOwnerAction) return;
  deferAction.disabled = true;
  deferAction.textContent = "Saved for later";
  if (homeAttention) homeAttention.textContent = "Your action is still safely waiting; nothing was approved or advanced.";
});
workOpenAction?.addEventListener("click", openCurrentOwnerAction);

async function loadStatus(): Promise<void> {
  if (!statusElement) return;
  try {
    const result = await invokeForge<KernelStatus>("kernel_status");
    statusElement.textContent = result.mode === "local-only"
      ? "Forge is connected locally, project history is verified, and technical records remain available on demand."
      : "Forge is connected and project history is available.";
  } catch {
    statusElement.textContent = "Forge desktop shell is running outside Tauri.";
  }
}

async function loadAtlas(): Promise<void> {
  if (!atlasSummary || !atlasMilestones || !atlasSystems) return;
  try {
    const atlas = await invokeForge<ProjectAtlas>("project_atlas");
    const active = atlas.milestones.find((milestone) => milestone.status === "active");
    atlasSummary.textContent = active
      ? `Current stage: ${active.name}. ${atlas.project.vision}`
      : `${atlas.project.vision} No active stage requires attention.`;
    atlasMilestones.replaceChildren(...atlas.milestones.map((milestone) => {
      const item = document.createElement("li");
      item.dataset.status = milestone.status;
      item.title = `${milestone.name} — ${milestone.status}`;
      item.textContent = milestone.name;
      return item;
    }));
    atlasSystems.replaceChildren(...atlas.systems.map((system) => {
      const item = document.createElement("li");
      item.textContent = `${system.id} [${system.stream}] ${system.name}: ${system.purpose}`;
      return item;
    }));
  } catch (error) {
    atlasSummary.textContent = `Project Atlas is unavailable: ${String(error)}`;
  }
}

async function loadOperating(): Promise<void> {
  if (!operatingSummary || !operatingPolicies) return;
  try {
    const snapshot = await invokeForge<OperatingSnapshot>("operating_system_snapshot");
    const work = snapshot.active_checkpoint;
    if (operatingOwnerTitle) operatingOwnerTitle.textContent = work.state === "complete" ? "Continuity foundation verified" : "Continuity foundation in progress";
    if (operatingOwnerSummary) operatingOwnerSummary.textContent = `${work.objective} Forge will not cross the next owner boundary automatically.`;
    operatingSummary.textContent = `${work.batch_id} [${work.state}/${work.substage_id}]: ${work.objective} Route: ${work.atlas_route.milestone}; risk/research: ${work.risk_level}/${work.research_gate}; next: ${work.next_action} Evidence catalogue: ${snapshot.catalogue_available ? "available" : "fallback transcript search"}.`;
    operatingPolicies.replaceChildren(...snapshot.policy.policies.filter((policy) => policy.status === "approved").map((policy) => {
      const item = document.createElement("li"); item.textContent = `${policy.id}: ${policy.rule}`; return item;
    }));
  } catch (error) { operatingSummary.textContent = `Operating state is unavailable: ${String(error)}`; }
}

function showCodexCapture(status: CodexCaptureStatus): void {
  if (!codexCaptureSummary) return;
  const error = status.last_error ? ` Last issue: ${status.last_error}` : "";
  codexCaptureSummary.textContent = `${status.state}: ${status.sessions} session(s); ${status.captured_messages} message(s) captured this scan; ${status.skipped_records} non-visible/internal record(s) skipped; ${status.paused_sources} source(s) paused safely.${error}`;
  const plainState = status.last_error ? "Needs attention" : status.state === "running" ? "Up to date" : status.state === "paused" ? "Paused" : "Checking";
  if (captureOwnerState) captureOwnerState.textContent = plainState;
  if (captureOwnerSummary) captureOwnerSummary.textContent = status.last_error
    ? "Forge could not finish checking conversation history. Open Technical details for the exact issue."
    : status.state === "running"
      ? "Visible conversation history is being recorded locally and is ready for Forge to use as evidence."
      : status.state === "paused"
        ? "Automatic conversation capture is paused. Existing project history remains safe."
        : "Forge is checking the local conversation history. If this state persists, use Check again.";
  if (captureChip) captureChip.textContent = status.state === "running" ? "● CAPTURE ON" : status.state === "paused" ? "○ CAPTURE PAUSED" : "○ CAPTURE CHECKING";
}

async function loadCodexCapture(): Promise<void> {
  try {
    showCodexCapture(await invokeForge<CodexCaptureStatus>("codex_capture_status"));
  } catch {
    if (codexCaptureSummary) codexCaptureSummary.textContent = "Automatic capture status is available only inside the local Forge desktop app.";
  }
}

async function loadDossier(): Promise<void> {
  if (!dossierSummary || !candidateList) return;
  try {
    const dossier = await invokeForge<DossierSnapshot>("dossier_snapshot");
    const activeApplications = dossier.applications.filter((application) => !application.rolled_back).length;
    dossierSummary.textContent = `Objects: ${dossier.object_count}; events: ${dossier.event_count}; candidates: ${dossier.candidates.length}; active applications: ${activeApplications}; rolled back: ${dossier.applications.length - activeApplications}.`;
    const visible = dossier.candidates.slice(0, 25);
    candidateList.replaceChildren(...visible.map((candidate) => {
      const item = document.createElement("li");
      item.textContent = `${candidate.state} candidate ${candidate.id} (evidence ${candidate.evidence_id}; history events ${candidate.history_event_count})`;
      return item;
    }));
    if (candidateMore) candidateMore.textContent = dossier.candidates.length > visible.length
      ? `Showing 25 of ${dossier.candidates.length} technical candidates.`
      : `${dossier.candidates.length} technical candidates.`;
  } catch {
    dossierSummary.textContent = "Dossier is available only inside the local Forge desktop app.";
  }
}

async function loadReferenceStudio(): Promise<void> {
  if (!referenceStudioSummary || !referenceStudioRecords || !referenceStudioGaps) return;
  try {
    const view = await invokeForge<ReferenceStudioSnapshot>("reference_studio_snapshot", { expectedSchemaVersion: 1 });
    const records = view.records;
    const compatibility = view.compatibility === "compatible" ? "schema compatible" : `VERSION MISMATCH: requested ${view.requested_schema_version}, local ${view.projection_schema_version}`;
    referenceStudioSummary.textContent = `${view.read_only ? "Read-only" : "INVALID MUTABLE VIEW"}; ${compatibility}; source ${view.projection_source}; verified ${new Date(view.verified_at_ms).toISOString()}. Proof receipts: ${records.proof_receipts.length}; work packages: ${records.work_packages.length}; gates: ${records.gate_receipts.length}; blockers: ${records.blockers.length}; rollbacks: ${records.rollbacks.length}.`;
    const items: HTMLLIElement[] = [];
    for (const receipt of records.proof_receipts) {
      const item = document.createElement("li");
      const failure = receipt.failure_classification ? ` Failure: ${receipt.failure_classification}.` : "";
      item.textContent = `${receipt.status.toUpperCase()} proof ${receipt.system_id}/${receipt.proof_id} (schema ${receipt.schema_version}; fixture ${receipt.fixture_id}).${failure} Inputs: ${receipt.input_refs.join(", ")}. Outputs: ${receipt.output_refs.join(", ")}. Equivalence: ${receipt.equivalence_method}. Limitations: ${receipt.limitations.join("; ")}.`;
      items.push(item);
    }
    for (const gate of records.gate_receipts) {
      const item = document.createElement("li");
      const failure = gate.failure_reason ? ` Failure: ${gate.failure_reason}.` : "";
      item.textContent = `${gate.outcome.toUpperCase()} gate ${gate.id}: ${gate.work_package_id} ${gate.from_stage} -> ${gate.to_stage}.${failure} Evidence: ${gate.evidence_ids.join(", ")}.`;
      items.push(item);
    }
    for (const blocker of records.blockers) {
      const item = document.createElement("li");
      item.textContent = `${blocker.status.toUpperCase()} ${blocker.blocker_type} blocker ${blocker.id}: ${blocker.requirement} (stage ${blocker.affected_stage}).`;
      items.push(item);
    }
    for (const rollback of records.rollbacks) {
      const item = document.createElement("li");
      item.textContent = `Rollback ${rollback.id} retains failed gate ${rollback.gate_receipt_id} for ${rollback.affected_artifact}: ${rollback.reason}.`;
      items.push(item);
    }
    referenceStudioRecords.replaceChildren(...items);
    referenceStudioGaps.replaceChildren(...records.source_gaps.map((gap) => {
      const item = document.createElement("li");
      item.textContent = `${gap.id}: ${gap.availability}/${gap.freshness}; ${gap.location}. Limitation: ${gap.limitations}. No missing details were inferred.`;
      return item;
    }));
  } catch (error) {
    referenceStudioSummary.textContent = `Reference Studio is unavailable: ${String(error)}`;
    referenceStudioRecords.replaceChildren();
    referenceStudioGaps.replaceChildren();
  }
}

let currentViewport: ReferenceViewportSnapshot | null = null;
let currentViewportBundle: ControlledStimulusBundle | null = null;

const reviewSteps = [
  { title: "Connection check", instruction: "Compare the reference with the altered version and check whether the parts remain connected in the expected places." },
  { title: "Shape check", instruction: "Check whether the reference keeps a clear, readable outline instead of collapsing into a flat shape." },
  { title: "Pose check", instruction: "Move the pose frame and check whether the reference joint stays in the expected position." },
];

function progressKey(bundle: ControlledStimulusBundle): string {
  return `forge-visual-review:${bundle.base_scene_fingerprint}`;
}

function receiptsKey(bundle: ControlledStimulusBundle): string {
  return `${progressKey(bundle)}:receipts`;
}

function completedReviewSteps(bundle: ControlledStimulusBundle): number[] {
  try {
    const parsed = JSON.parse(localStorage.getItem(progressKey(bundle)) ?? "[]");
    return Array.isArray(parsed) ? parsed.filter((value) => Number.isInteger(value) && value >= 0 && value < 3) : [];
  } catch {
    return [];
  }
}

function updateGuidedReview(): void {
  if (!currentViewportBundle || !referenceObservationPair) return;
  const completed = completedReviewSteps(currentViewportBundle);
  const firstIncomplete = reviewSteps.findIndex((_, index) => !completed.includes(index));
  if (!reviewPreviewPinned) currentReviewStep = firstIncomplete === -1 ? 2 : firstIncomplete;
  const step = reviewSteps[currentReviewStep];
  if (reviewStepLabel) reviewStepLabel.textContent = firstIncomplete === -1 ? `REVIEW COMPLETE — VIEW ${currentReviewStep + 1} OF ${reviewSteps.length}` : `STEP ${currentReviewStep + 1} OF ${reviewSteps.length}`;
  if (reviewProgressFill) reviewProgressFill.style.width = `${Math.round((completed.length / reviewSteps.length) * 100)}%`;
  if (reviewCheckTitle) reviewCheckTitle.textContent = step.title;
  if (reviewCheckInstruction) reviewCheckInstruction.textContent = step.instruction;
  if (reviewPreviousStep) reviewPreviousStep.disabled = currentReviewStep === 0;
  if (reviewNextStep) reviewNextStep.disabled = currentReviewStep === reviewSteps.length - 1;
  const placeholder = document.createElement("option");
  placeholder.value = "";
  const currentIsActionable = firstIncomplete === currentReviewStep;
  placeholder.textContent = firstIncomplete === -1 ? "Review complete" : currentIsActionable ? "Choose when ready" : "Preview only — finish the earlier check first";
  const stimulus = currentViewportBundle.stimuli[currentReviewStep + 1];
  const option = document.createElement("option");
  option.value = String(currentReviewStep);
  option.textContent = stimulus ? `Use ${controlLabel(stimulus.control).split(" — ")[0].toLocaleLowerCase()}` : "Comparison unavailable";
  referenceObservationPair.replaceChildren(placeholder, ...(currentIsActionable ? [option] : []));
  referenceObservationPair.value = "";
  referenceObservationPair.disabled = !currentIsActionable;
  if (currentOwnerAction?.action_id === "F5") {
    currentOwnerAction.progress_current = completed.length;
    if (homeActionProgress) homeActionProgress.textContent = `${completed.length} OF ${reviewSteps.length}`;
    if (homeProgressFill) homeProgressFill.style.width = `${Math.round((completed.length / reviewSteps.length) * 100)}%`;
  }
}

function controlLabel(control: ControlledStimulusBundle["stimuli"][number]["control"]): string {
  switch (control) {
    case "broken_connection": return "Connection check — compare with a deliberately disconnected shape";
    case "silhouette_collapse": return "Shape check — compare with a deliberately collapsed outline";
    case "articulation_drift": return "Pose check — compare with a deliberately shifted joint";
    default: return "Reference shape";
  }
}

function drawReferenceViewport(snapshot: ReferenceViewportSnapshot, frameIndex: number): void {
  if (!referenceViewportCanvas) return;
  const context = referenceViewportCanvas.getContext("2d");
  const frame = snapshot.frames[frameIndex];
  if (!context || !frame) return;
  const width = referenceViewportCanvas.width;
  const height = referenceViewportCanvas.height;
  context.clearRect(0, 0, width, height);
  context.fillStyle = "#090b0f";
  context.fillRect(0, 0, width, height);
  const panelWidth = width / frame.views.length;

  frame.views.forEach((view, panelIndex) => {
    const centerX = panelWidth * panelIndex + panelWidth / 2;
    const centerY = height / 2 + 15;
    const pointById = new Map(view.points.map((point) => [point.id, point]));
    context.strokeStyle = "#4b4f58";
    context.strokeRect(panelWidth * panelIndex + 8, 8, panelWidth - 16, height - 16);
    context.fillStyle = "#dedde0";
    context.font = "16px system-ui";
    context.fillText(`${view.view.toUpperCase()} — frame ${frame.index}`, panelWidth * panelIndex + 20, 34);
    for (const edge of snapshot.edges) {
      const from = pointById.get(edge.from);
      const to = pointById.get(edge.to);
      if (!from || !to) continue;
      context.beginPath();
      context.moveTo(centerX + from.x * 0.55, centerY - from.y * 0.55);
      context.lineTo(centerX + to.x * 0.55, centerY - to.y * 0.55);
      context.strokeStyle = edge.role === "articulation" ? "#f0b44d" : edge.role === "support" ? "#75d47f" : "#e32636";
      context.lineWidth = 5;
      context.stroke();
    }
    for (const point of view.points) {
      context.beginPath();
      context.arc(centerX + point.x * 0.55, centerY - point.y * 0.55, 6, 0, Math.PI * 2);
      context.fillStyle = "#f8fafc";
      context.fill();
    }
  });
}

async function loadReferenceViewport(): Promise<void> {
  if (!referenceViewportSummary || !referenceViewportFrame || !referenceViewportStimulus || !referenceViewportLimitations) return;
  try {
    const bundle = await invokeForge<ControlledStimulusBundle>("reference_viewport_stimulus_bundle");
    const snapshot = bundle.stimuli[0]?.snapshot;
    if (!snapshot) throw new Error("controlled stimulus bundle is empty");
    currentViewportBundle = bundle;
    currentViewport = snapshot;
    referenceViewportStimulus.replaceChildren(...bundle.stimuli.map((stimulus, index) => {
      const option = document.createElement("option");
      option.value = String(index);
      option.textContent = controlLabel(stimulus.control);
      return option;
    }));
    referenceViewportFrame.min = "0";
    referenceViewportFrame.max = String(Math.max(0, snapshot.frames.length - 1));
    referenceViewportFrame.value = "0";
    referenceViewportSummary.textContent = snapshot.read_only
      ? "Ready. Use the two buttons to switch between the reference and the altered version for this step."
      : "This view failed its read-only safety check and cannot be reviewed.";
    referenceViewportLimitations.replaceChildren(...[...bundle.limitations, ...snapshot.limitations].map((limitation) => {
      const item = document.createElement("li");
      item.textContent = limitation;
      return item;
    }));
    drawReferenceViewport(snapshot, 0);
    updateGuidedReview();
  } catch (error) {
    referenceViewportSummary.textContent = `Built-in reference viewport is unavailable: ${String(error)}`;
    currentViewport = null;
    currentViewportBundle = null;
  }
}

async function loadOwnerBrief(): Promise<void> {
  if (!briefSummary || !briefList) return;
  try {
    const brief = await invokeForge<OwnerBrief>("owner_brief");
    const suffix = brief.truncated ? " Showing the first five only." : "";
    briefSummary.textContent = `${brief.pending_decision_count} background technical candidates.${suffix} They do not enter the owner queue unless a separate canonical gate makes one actionable.`;
    briefList.replaceChildren(...brief.visible_decisions.map((item) => {
      const entry = document.createElement("li");
      entry.textContent = `${item.state}: ${item.candidate_id} (evidence ${item.evidence_id})`;
      return entry;
    }));
  } catch {
    briefSummary.textContent = "Owner brief is available only inside the local Forge desktop app.";
  }
}

form?.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!sourceInput || !transcriptInput || !receiptElement) return;
  receiptElement.textContent = "Recording explicitly supplied evidence...";
  try {
    const report = await invokeForge<ImportReport>("import_labeled_transcript", {
      sourceId: sourceInput.value,
      transcript: transcriptInput.value,
    });
    const action = report.already_recorded ? "Already recorded; no new evidence was created." : "Recorded new evidence.";
    receiptElement.textContent = `${action} Messages: ${report.message_count} from ${report.source_id}. Candidates: ${report.candidate_count}; correction flags: ${report.correction_intents}; approval-language flags: ${report.approval_intents}. No approval or promotion was performed.`;
    await loadStatus();
    await loadDossier();
    await loadOwnerBrief();
    await loadOwnerDashboard();
  } catch (error) {
    receiptElement.textContent = `Import was not accepted: ${String(error)}`;
  }
});

refreshDossier?.addEventListener("click", () => void loadDossier());
refreshReferenceStudio?.addEventListener("click", () => void loadReferenceStudio());
refreshReferenceViewport?.addEventListener("click", () => void loadReferenceViewport());
referenceViewportFrame?.addEventListener("input", () => {
  if (currentViewport && referenceViewportFrame) drawReferenceViewport(currentViewport, Number(referenceViewportFrame.value));
});
referenceViewportStimulus?.addEventListener("change", () => {
  if (!currentViewportBundle || !referenceViewportStimulus || !referenceViewportFrame) return;
  const stimulus = currentViewportBundle.stimuli[Number(referenceViewportStimulus.value)];
  if (!stimulus) return;
  currentViewport = stimulus.snapshot;
  referenceViewportFrame.max = String(Math.max(0, stimulus.snapshot.frames.length - 1));
  referenceViewportFrame.value = "0";
  drawReferenceViewport(stimulus.snapshot, 0);
});
showReference?.addEventListener("click", () => {
  if (!currentViewportBundle || !referenceViewportStimulus || !referenceViewportFrame) return;
  const stimulus = currentViewportBundle.stimuli[0];
  referenceViewportStimulus.value = "0";
  currentViewport = stimulus.snapshot;
  referenceViewportFrame.max = String(Math.max(0, stimulus.snapshot.frames.length - 1));
  referenceViewportFrame.value = "0";
  drawReferenceViewport(stimulus.snapshot, 0);
});
showCurrentComparison?.addEventListener("click", () => {
  if (!currentViewportBundle || !referenceViewportStimulus || !referenceViewportFrame) return;
  const stimulusIndex = currentReviewStep + 1;
  const stimulus = currentViewportBundle.stimuli[stimulusIndex];
  if (!stimulus) return;
  referenceViewportStimulus.value = String(stimulusIndex);
  currentViewport = stimulus.snapshot;
  referenceViewportFrame.max = String(Math.max(0, stimulus.snapshot.frames.length - 1));
  referenceViewportFrame.value = "0";
  drawReferenceViewport(stimulus.snapshot, 0);
});
reviewPreviousStep?.addEventListener("click", () => {
  if (currentReviewStep === 0) return;
  currentReviewStep -= 1;
  reviewPreviewPinned = true;
  updateGuidedReview();
  showReference?.click();
});
reviewNextStep?.addEventListener("click", () => {
  if (currentReviewStep >= reviewSteps.length - 1) return;
  currentReviewStep += 1;
  reviewPreviewPinned = true;
  updateGuidedReview();
  showReference?.click();
});
referenceObservationPair?.addEventListener("change", () => {
  if (!currentViewportBundle || !referenceViewportStimulus || !referenceViewportFrame || referenceObservationPair.value === "") return;
  const stimulusIndex = Number(referenceObservationPair.value) + 1;
  const stimulus = currentViewportBundle.stimuli[stimulusIndex];
  if (!stimulus) return;
  referenceViewportStimulus.value = String(stimulusIndex);
  currentViewport = stimulus.snapshot;
  referenceViewportFrame.max = String(Math.max(0, stimulus.snapshot.frames.length - 1));
  referenceViewportFrame.value = "0";
  drawReferenceViewport(stimulus.snapshot, 0);
});
referenceObservationForm?.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!currentViewportBundle || !referenceObservationPair || !referenceObservationOutcome || !referenceObservationConfidence || !referenceObservationReceipt) return;
  if (referenceObservationPair.value === "" || referenceObservationOutcome.value === "" || referenceObservationConfidence.value === "") {
    referenceObservationReceipt.textContent = "Choose a comparison, an observed outcome, and confidence before creating a receipt.";
    return;
  }
  referenceObservationReceipt.textContent = "Checking and saving your observation…";
  try {
    const completedStep = Number(referenceObservationPair.value);
    const receipt = await invokeForge<OwnerObservationReceipt>("record_reference_viewport_observation", {
      input: {
        expected_base_scene_fingerprint: currentViewportBundle.base_scene_fingerprint,
        pair_index: Number(referenceObservationPair.value),
        outcome: referenceObservationOutcome.value,
        confidence: Number(referenceObservationConfidence.value),
      },
    });
    const completed = completedReviewSteps(currentViewportBundle);
    if (!completed.includes(completedStep)) completed.push(completedStep);
    localStorage.setItem(progressKey(currentViewportBundle), JSON.stringify(completed.sort()));
    const savedReceipts = JSON.parse(localStorage.getItem(receiptsKey(currentViewportBundle)) ?? "{}");
    savedReceipts[String(completedStep)] = receipt;
    localStorage.setItem(receiptsKey(currentViewportBundle), JSON.stringify(savedReceipts));
    referenceObservationReceipt.textContent = completed.length === reviewSteps.length
      ? "All three observations are saved locally. Nothing was approved, promoted, or changed automatically."
      : "Observation saved. The next visual check is ready.";
    if (referenceObservationTechnical && referenceObservationTechnicalText) {
      referenceObservationTechnical.hidden = false;
      referenceObservationTechnicalText.textContent = `${receipt.status}; ${receipt.control}/${receipt.outcome}; confidence ${receipt.confidence}; receipt SHA-256 ${receipt.receipt_fingerprint}; authority effect ${receipt.authority_effect}.`;
    }
    referenceObservationOutcome.value = "";
    referenceObservationConfidence.value = "";
    reviewPreviewPinned = false;
    updateGuidedReview();
    showReference?.click();
  } catch (error) {
    referenceObservationReceipt.textContent = `Observation receipt was rejected: ${String(error)}`;
  }
});
refreshAtlas?.addEventListener("click", () => void loadAtlas());
refreshOperating?.addEventListener("click", () => void loadOperating());
refreshBrief?.addEventListener("click", () => void loadOwnerBrief());
refreshCodexCapture?.addEventListener("click", () => void loadCodexCapture());
pauseCodexCapture?.addEventListener("click", async () => {
  try { showCodexCapture(await invokeForge<CodexCaptureStatus>("pause_codex_capture")); }
  catch (error) { if (codexCaptureSummary) codexCaptureSummary.textContent = `Capture was not paused: ${String(error)}`; }
});
resumeCodexCapture?.addEventListener("click", async () => {
  try {
    showCodexCapture(await invokeForge<CodexCaptureStatus>("resume_codex_capture"));
    await loadStatus(); await loadDossier(); await loadOwnerBrief();
  } catch (error) { if (codexCaptureSummary) codexCaptureSummary.textContent = `Capture was not resumed: ${String(error)}`; }
});

authorizationForm?.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!authorizationAction || !authorizationCandidate || !authorizationConfirmation || !authorizationReceipt) return;
  authorizationReceipt.textContent = "Recording explicit authorization...";
  try {
    const receipt = await invokeForge<AuthorizationReceipt>("authorize_candidate", {
      action: authorizationAction.value,
      candidateId: authorizationCandidate.value,
      confirmation: authorizationConfirmation.value,
      correctionEvidenceId: authorizationCorrection?.value || null,
      replacementCandidateId: authorizationReplacement?.value || null,
    });
    authorizationReceipt.textContent = `${receipt.action} candidate ${receipt.candidate_id}. Ledger event: ${receipt.event_id}. This did not apply code or write a repository file.`;
    authorizationConfirmation.value = "";
    await loadStatus();
    await loadDossier();
    await loadOwnerBrief();
  } catch (error) {
    authorizationReceipt.textContent = `Authorization was not accepted: ${String(error)}`;
  }
});

function updateAuthorizationHint() {
  if (!authorizationAction || !authorizationCandidate || !authorizationHint) return;
  const candidate = authorizationCandidate.value || "<candidate ID>";
  if (authorizationAction.value === "supersede") {
    const correction = authorizationCorrection?.value || "<correction evidence ID>";
    const replacement = authorizationReplacement?.value;
    authorizationHint.textContent = replacement
      ? `Exact phrase: SUPERSEDE ${candidate} USING ${correction} WITH ${replacement}`
      : `Exact phrase: SUPERSEDE ${candidate} USING ${correction}`;
    return;
  }
  authorizationHint.textContent = `Exact phrase: ${authorizationAction.value.toUpperCase()} ${candidate}`;
}

[authorizationAction, authorizationCandidate, authorizationCorrection, authorizationReplacement]
  .forEach((element) => element?.addEventListener("input", updateAuthorizationHint));
updateAuthorizationHint();

backupButton?.addEventListener("click", async () => {
  if (!backupReceipt) return;
  backupButton.disabled = true;
  backupReceipt.textContent = "Creating and verifying local backup...";
  try {
    const receipt = await invokeForge<BackupReceipt>("create_local_backup");
    backupReceipt.textContent = `Verified local backup: ${receipt.path}. SHA-256: ${receipt.sha256}. Bytes: ${receipt.bytes}; objects: ${receipt.object_count}; events: ${receipt.event_count}; candidates: ${receipt.candidate_count}.`;
  } catch (error) {
    backupReceipt.textContent = `Backup was not verified: ${String(error)}`;
  } finally {
    backupButton.disabled = false;
  }
});

codeForm?.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!codeSource || !codePath || !codeLanguage || !codeText || !codeReceipt) return;
  codeReceipt.textContent = "Recording non-executable code evidence...";
  try {
    const receipt = await invokeForge<CodeAdmissionReceipt>("admit_pasted_code", {
      sourceId: codeSource.value,
      relativePath: codePath.value,
      language: codeLanguage.value,
      code: codeText.value,
    });
    const action = receipt.already_recorded ? "Already recorded; no new evidence was created." : "Recorded code evidence.";
    codeReceipt.textContent = `${action} Candidate: ${receipt.candidate}. Target path: ${receipt.relative_path}. No file was written, code was not run, and no approval or promotion was performed.`;
    if (previewCandidate) previewCandidate.value = receipt.candidate;
    await loadStatus();
    await loadDossier();
    await loadOwnerBrief();
  } catch (error) {
    codeReceipt.textContent = `Code was not admitted: ${String(error)}`;
  }
});

previewButton?.addEventListener("click", async () => {
  if (!previewCandidate || !previewSummary || !codePreview) return;
  previewSummary.textContent = "Loading read-only code preview...";
  try {
    const preview = await invokeForge<CodePreview>("preview_code_candidate", { candidateId: previewCandidate.value });
    previewSummary.textContent = `Read-only preview: ${preview.relative_path} (${preview.language}). No file has been written.`;
    codePreview.textContent = preview.code;
  } catch (error) {
    previewSummary.textContent = `Code preview was not available: ${String(error)}`;
    codePreview.textContent = "";
  }
});

applyForm?.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!applyCandidate || !applyConfirmation || !applyReceipt) return;
  applyReceipt.textContent = "Applying promoted candidate to local staging workspace...";
  try {
    const receipt = await invokeForge<AppliedCodeReceipt>("apply_promoted_code", {
      candidateId: applyCandidate.value,
      confirmation: applyConfirmation.value,
    });
    applyReceipt.textContent = `Created staging file: ${receipt.path}. Ledger event: ${receipt.event_id}. The Forge source repository was not changed.`;
    applyConfirmation.value = "";
    await loadStatus();
    void loadWorkspace();
  } catch (error) {
    applyReceipt.textContent = `Staging application was not accepted: ${String(error)}`;
  }
});

rollbackForm?.addEventListener("submit", async (event) => {
  event.preventDefault();
  if (!rollbackEvent || !rollbackConfirmation || !rollbackReceipt) return;
  rollbackReceipt.textContent = "Rolling back application and verifying workspace...";
  try {
    const receipt = await invokeForge<AppliedCodeReceipt>("rollback_application", {
      applicationEventId: rollbackEvent.value,
      confirmation: rollbackConfirmation.value,
    });
    rollbackReceipt.textContent = `Rollback recorded: ${receipt.event_id}. Workspace path: ${receipt.path}.`;
    rollbackConfirmation.value = "";
    await loadStatus();
    await loadDossier();
  } catch (error) {
    rollbackReceipt.textContent = `Rollback was not accepted: ${String(error)}`;
  }
});

async function loadWorkspace(): Promise<void> {
  if (!workspaceSummary || !workspaceFiles) return;
  try {
    const files = await invokeForge<WorkspaceFile[]>("staging_inventory");
    workspaceSummary.textContent = `Read-only staging inventory: ${files.length} files.`;
    workspaceFiles.replaceChildren(...files.map((file) => {
      const item = document.createElement("li");
      item.textContent = `${file.relative_path} (${file.bytes} bytes; SHA-256 ${file.sha256})`;
      return item;
    }));
  } catch {
    workspaceSummary.textContent = "Workspace inventory is available only inside the local Forge desktop app.";
  }
}

workspaceButton?.addEventListener("click", () => void loadWorkspace());

goToReview?.addEventListener("click", () => {
  openCurrentOwnerAction();
  window.setTimeout(() => referenceObservationPair?.focus(), 350);
});

toggleDetails?.addEventListener("click", () => {
  if (!mainElement || !toggleDetails) return;
  const showing = mainElement.classList.toggle("show-details");
  toggleDetails.textContent = showing ? "Hide technical details" : "Show technical details";
  toggleDetails.setAttribute("aria-expanded", String(showing));
});

projectButton?.addEventListener("click", async () => {
  if (!projectSummary) return;
  try {
    const files = await invokeForge<WorkspaceFile[]>("project_inventory");
    const bytes = files.reduce((total, file) => total + file.bytes, 0);
    projectSummary.textContent = `Approved Forge workspace: ${files.length} inventoried files, ${bytes} bytes. Read-only; no Forge source files were changed.`;
  } catch (error) {
    projectSummary.textContent = `Approved workspace inventory was not available: ${String(error)}`;
  }
});

void loadStatus();
void loadAtlas();
void loadOperating();
void loadCodexCapture();
void loadDossier();
void loadReferenceStudio();
void loadReferenceViewport();
void loadOwnerBrief();
void loadKnowledge();
void loadOwnerDashboard();
navigate(window.location.hash.slice(1) || "home");
