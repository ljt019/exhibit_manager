export const calculateTimeUntilExpiration = (endDate: string) => {
  const end = new Date(endDate);
  const now = new Date();
  const diff = end.getTime() - now.getTime();
  const days = Math.ceil(diff / (1000 * 3600 * 24));
  return days > 0 ? `${days} days` : "Expired";
};
