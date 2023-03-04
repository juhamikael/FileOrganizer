export const readBackupStatus = async () => {
  const response = await fetch("config.json");
  const config = await response.json();
  const enableBackup: boolean = config.enableBackup;
  return enableBackup;
};
