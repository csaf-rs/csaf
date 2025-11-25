import prettyBytes from 'pretty-bytes';
import InfoCard from './InfoCard';

interface DocumentInfoProps {
  filename: string;
  size: number | null;
}

function DocumentInfo({ filename, size }: DocumentInfoProps) {
  return (
    <InfoCard
      title="Document Information"
      items={[
        { label: 'Filename', value: filename },
        { label: 'Size', value: size !== null ? prettyBytes(size) : 'N/A' },
      ]}
      columns={2}
    />
  );
}

export default DocumentInfo;
