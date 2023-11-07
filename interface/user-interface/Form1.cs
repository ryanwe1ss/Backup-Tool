namespace user_interface
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();

            comboBox1.SelectedIndex = 0;
            dataGridView1.CellContentClick += DataGridView1_CellContentClick;
        }

        private void button1_Click(object sender, EventArgs e)
        {
            
        }

        private void button2_Click(object sender, EventArgs e)
        {
            if (openFileDialog1.ShowDialog() == DialogResult.OK)
            {
                string[] selectedFilePaths = openFileDialog1.FileNames;
                foreach (string filePath in selectedFilePaths)
                {
                    string fileName = Path.GetFileName(filePath);
                    string fileLength = FormatFileLength(new FileInfo(filePath).Length);

                    dataGridView1.Rows.Add(
                        fileName,
                        filePath,
                        fileLength
                    );
                }
            }
        }

        private void comboBox1_SelectedIndexChanged(object sender, EventArgs e)
        {
            usernameTextBox.Enabled = (string)comboBox1.SelectedItem == "FTP";
            passwordTextBox.Enabled = (string)comboBox1.SelectedItem == "FTP";
        }

        private void DataGridView1_CellContentClick(object sender, DataGridViewCellEventArgs e)
        {
            if (dataGridView1.Columns[e.ColumnIndex].Name == "FileRemove")
            {
                dataGridView1.Rows.RemoveAt(e.RowIndex);
            }
        }

        private string FormatFileLength(long fileLength)
        {
            const long KB = 1024;
            const long MB = KB * 1024;

            if (fileLength >= MB)
            {
                return $"{fileLength / (double)MB:F2} MB";
            }
            else if (fileLength >= KB)
            {
                return $"{fileLength / (double)KB:F2} KB";
            }
            else
            {
                return $"{fileLength} B";
            }
        }
    }
}