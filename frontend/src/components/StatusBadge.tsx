import { StageStatus } from '../types';
import './StatusBadge.css';

interface StatusBadgeProps {
  status: StageStatus;
}

const statusConfig: Record<StageStatus, { label: string; className: string }> = {
  pending: { label: 'Pendente', className: 'pending' },
  running: { label: 'Rodando', className: 'running' },
  success: { label: 'Sucesso', className: 'success' },
  failed: { label: 'Falhou', className: 'failed' },
};

export function StatusBadge({ status }: StatusBadgeProps) {
  const config = statusConfig[status];
  return <span className={`status-badge ${config.className}`}>{config.label}</span>;
}
