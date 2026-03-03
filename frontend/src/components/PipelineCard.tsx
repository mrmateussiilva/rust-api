import { Pipeline } from '../types';
import { StatusBadge } from './StatusBadge';
import './PipelineCard.css';

interface PipelineCardProps {
  pipeline: Pipeline;
  onDelete: (id: string) => void;
}

export function PipelineCard({ pipeline, onDelete }: PipelineCardProps) {
  return (
    <div className="pipeline-card">
      <div className="pipeline-header">
        <h3>{pipeline.name}</h3>
        <button className="delete-btn" onClick={() => onDelete(pipeline.id)}>
          ×
        </button>
      </div>
      <p className="pipeline-description">{pipeline.description}</p>
      <div className="pipeline-stages">
        {pipeline.stages.map((stage) => (
          <div key={stage.id} className="stage-item">
            <span className="stage-order">{stage.order}</span>
            <span className="stage-name">{stage.name}</span>
            <StatusBadge status={stage.status} />
          </div>
        ))}
        {pipeline.stages.length === 0 && (
          <p className="no-stages">Nenhum estágio configurado</p>
        )}
      </div>
      <div className="pipeline-footer">
        <span className="pipeline-date">
          Criado em {new Date(pipeline.createdAt).toLocaleDateString('pt-BR')}
        </span>
      </div>
    </div>
  );
}
