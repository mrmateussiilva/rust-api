import { useEffect, useState } from 'react';
import { Pipeline } from './types';
import { api } from './services/api';
import { PipelineCard } from './components/PipelineCard';
import { CreatePipelineModal } from './components/CreatePipelineModal';
import './App.css';

function App() {
  const [pipelines, setPipelines] = useState<Pipeline[]>([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchPipelines = async () => {
    try {
      const data = await api.pipelines.getAll();
      setPipelines(data);
      setError(null);
    } catch (err) {
      setError('Erro ao carregar esteiras. Verifique se a API está rodando.');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPipelines();
  }, []);

  const handleCreatePipeline = async (name: string, description: string) => {
    try {
      const newPipeline = await api.pipelines.create({ name, description });
      setPipelines([...pipelines, newPipeline]);
    } catch (err) {
      console.error('Erro ao criar esteira:', err);
    }
  };

  const handleDeletePipeline = async (id: string) => {
    if (!confirm('Tem certeza que deseja excluir esta esteira?')) return;
    try {
      await api.pipelines.delete(id);
      setPipelines(pipelines.filter((p) => p.id !== id));
    } catch (err) {
      console.error('Erro ao excluir esteira:', err);
    }
  };

  return (
    <div className="app">
      <header className="header">
        <h1>Esteiras</h1>
        <button className="btn-primary" onClick={() => setIsModalOpen(true)}>
          + Nova Esteira
        </button>
      </header>

      <main className="main">
        {loading && <p className="loading">Carregando...</p>}
        
        {error && (
          <div className="error">
            <p>{error}</p>
            <button onClick={fetchPipelines}>Tentar novamente</button>
          </div>
        )}

        {!loading && !error && pipelines.length === 0 && (
          <div className="empty">
            <p>Nenhuma esteira encontrada.</p>
            <p>Crie sua primeira esteira para começar.</p>
          </div>
        )}

        <div className="pipeline-grid">
          {pipelines.map((pipeline) => (
            <PipelineCard
              key={pipeline.id}
              pipeline={pipeline}
              onDelete={handleDeletePipeline}
            />
          ))}
        </div>
      </main>

      <CreatePipelineModal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        onSubmit={handleCreatePipeline}
      />
    </div>
  );
}

export default App;
