import { Pipeline, CreatePipelineDto, UpdatePipelineDto, CreateStageDto, Stage } from '../types';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

async function request<T>(endpoint: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_URL}${endpoint}`, {
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
    ...options,
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return response.json();
}

export const api = {
  pipelines: {
    getAll: () => request<Pipeline[]>('/pipelines'),
    getById: (id: string) => request<Pipeline>(`/pipelines/${id}`),
    create: (data: CreatePipelineDto) =>
      request<Pipeline>('/pipelines', { method: 'POST', body: JSON.stringify(data) }),
    update: (id: string, data: UpdatePipelineDto) =>
      request<Pipeline>(`/pipelines/${id}`, { method: 'PUT', body: JSON.stringify(data) }),
    delete: (id: string) =>
      request<void>(`/pipelines/${id}`, { method: 'DELETE' }),
  },
  stages: {
    create: (data: CreateStageDto) =>
      request<Stage>('/stages', { method: 'POST', body: JSON.stringify(data) }),
    updateStatus: (id: string, status: Stage['status']) =>
      request<Stage>(`/stages/${id}/status`, { method: 'PATCH', body: JSON.stringify({ status }) }),
    delete: (id: string) =>
      request<void>(`/stages/${id}`, { method: 'DELETE' }),
  },
};
