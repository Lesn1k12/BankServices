'use client'

export default function Loading() {
    return (
      <div style={{ textAlign: 'center', marginTop: '50px' }}>
        <div className="spinner" style={{ margin: 'auto', width: '50px', height: '50px', border: '5px solid #ccc', borderTop: '5px solid #333', borderRadius: '50%', animation: 'spin 1s linear infinite' }}></div>
        <p>Loading...</p>
        <style jsx>{`
          @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
          }
        `}</style>
      </div>
    );
  }