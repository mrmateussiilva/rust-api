export type StageStatus = 'pending' | 'running' | 'success' | 'failed';

export interface Stage {
  id: string;
  name: string;
  order: number;
  status: StageStatus;
  createdAt: string;
  updatedAt: string;
}

export interface Pipeline {
  id: string;
  name: string;
  description: string;
  stages: Stage[];
  createdAt: string;
  updatedAt: string;
}

export interface CreatePipelineDto {
  name: string;
  description: string;
}

export interface UpdatePipelineDto {
  name?: string;
  description?: string;
}

export interface CreateStageDto {
  pipelineId: string;
  name: string;
  order: number;
}
