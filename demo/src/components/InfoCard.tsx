interface InfoCardProps {
  title?: string;
  items: Array<{
    label: string;
    value: string | number;
  }>;
  columns?: 1 | 2 | 3 | 4;
}

function InfoCard({ title, items, columns = 2 }: InfoCardProps) {
  const gridCols = {
    1: 'grid-cols-1',
    2: 'grid-cols-1 sm:grid-cols-2',
    3: 'grid-cols-1 sm:grid-cols-3',
    4: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-4',
  };

  return (
    <div className="bg-white rounded-lg shadow-sm p-6 mb-6">
      {title && (
        <h3 className="text-lg font-semibold text-gray-900 mb-4">{title}</h3>
      )}
      <div className={`grid ${gridCols[columns]} gap-4`}>
        {items.map((item, index) => (
          <div key={index} className="bg-gray-50 rounded-lg p-4">
            <span className="text-sm text-gray-600">{item.label}</span>
            <p className="text-xl font-bold text-gray-900 capitalize">
              {item.value}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}

export default InfoCard;
